mod error;
mod listener;

pub use error::Error;
pub use listener::Listener;

use puniyu_event::{Event, EventType};
use smol_str::SmolStr;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

/// 监听器句柄，用于移除已注册的监听器
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ListenerId(u64);

impl ListenerId {
	pub const fn new(id: u64) -> Self {
		Self(id)
	}
	/// 获取监听器 ID
	pub const fn id(&self) -> u64 {
		self.0
	}
}

struct ListenerEntry {
	id: ListenerId,
	name: SmolStr,
	priority: u32,
	listener: Arc<dyn Listener>,
	once: bool,
}

struct Inner {
	accepting: AtomicBool,
	listeners: scc::HashMap<EventType, Vec<ListenerEntry>>,
	id_to_type: scc::HashMap<ListenerId, EventType>,
	next_id: AtomicU64,
}

/// 事件总线，支持按事件类型注册、移除和分发监听器。
///
/// 内部使用 [`scc::HashMap`] 实现无锁并发访问，不同事件类型的操作互不阻塞。
/// 监听器按优先级排序（数值越小越先执行），同优先级内按注册顺序调用。
///
/// # 用法
///
/// ```ignore
/// let emitter = EventEmitter::new();
/// emitter.start()?;
///
/// let id = emitter.on(EventType::Message, my_listener)?;
/// emitter.emit(event).await?;
///
/// emitter.off(id);
/// emitter.stop();
/// ```
#[derive(Clone)]
pub struct EventEmitter {
	inner: Arc<Inner>,
}

impl EventEmitter {
	/// 创建新的事件发射器
	pub fn new() -> Self {
		Self::default()
	}

	/// 启动发射器，开始接受事件注册和发射。
	pub fn start(&self) -> Result<(), Error> {
		self.inner.accepting.store(true, Ordering::Release);
		Ok(())
	}

	/// 停止发射器，清除所有监听器。
	pub fn stop(&self) {
		self.inner.accepting.store(false, Ordering::Release);
		self.inner.listeners.clear_sync();
		self.inner.id_to_type.clear_sync();
	}

	/// 注册持久监听器，返回 [`ListenerId`] 用于后续移除。
	///
	/// # 错误
	/// - 发射器未运行时返回 [`Error::NotRunning`]
	/// - 同名监听器已注册时返回 [`Error::AlreadyListening`]
	pub fn on<L: Listener + 'static>(
		&self,
		event_type: EventType,
		listener: L,
	) -> Result<ListenerId, Error> {
		self.register(event_type, listener, false)
	}

	/// 注册一次性监听器，触发一次后自动移除。
	///
	/// # 错误
	/// 同 [`EventEmitter::on`]。
	pub fn once<L: Listener + 'static>(
		&self,
		event_type: EventType,
		listener: L,
	) -> Result<ListenerId, Error> {
		self.register(event_type, listener, true)
	}

	/// 通过 [`ListenerId`] 移除监听器。返回是否找到并移除。
	pub fn off(&self, id: ListenerId) -> bool {
		let event_type = match self.inner.id_to_type.remove_sync(&id) {
			Some((_, event_type)) => event_type,
			None => return false,
		};
		self.inner.listeners.update_sync(&event_type, |_, entries| {
			entries.retain(|e| e.id != id);
		});
		true
	}

	/// 移除指定事件类型下的所有监听器。
	pub fn off_all(&self, event_type: EventType) {
		if let Some((_, entries)) = self.inner.listeners.remove_sync(&event_type) {
			for entry in entries {
				self.inner.id_to_type.remove_sync(&entry.id);
			}
		}
	}

	/// 发射事件，按优先级顺序依次调用匹配的监听器（同优先级按注册顺序）。
	///
	/// 一次性监听器在触发后自动移除。
	///
	/// # 错误
	/// 发射器未运行时返回 [`Error::NotRunning`]。
	pub async fn emit(&self, event: Event) -> Result<(), Error> {
		if !self.inner.accepting.load(Ordering::Acquire) {
			return Err(Error::NotRunning);
		}

		let event_type = event.event_type();

		let mut snapshot: Vec<(u32, Arc<dyn Listener>, bool)> = self
			.inner
			.listeners
			.read_sync(&event_type, |_, entries| {
				entries.iter().map(|e| (e.priority, Arc::clone(&e.listener), e.once)).collect()
			})
			.unwrap_or_default();
		snapshot.sort_by_key(|(p, _, _)| *p);

		if snapshot.is_empty() {
			return Ok(());
		}

		let mut fired_once = Vec::new();
		for (_, listener, is_once) in &snapshot {
			listener.handle(&event).await;
			if *is_once {
				fired_once.push(Arc::clone(listener));
			}
		}

		if !fired_once.is_empty() {
			let mut removed_ids: Vec<ListenerId> = Vec::new();
			self.inner.listeners.update_sync(&event_type, |_, entries| {
				entries.retain(|e| {
					let remove = fired_once.iter().any(|l| Arc::ptr_eq(l, &e.listener));
					if remove {
						removed_ids.push(e.id);
					}
					!remove
				});
			});
			for id in removed_ids {
				self.inner.id_to_type.remove_sync(&id);
			}
		}

		Ok(())
	}

	/// 查询指定事件类型的监听器数量
	pub fn len(&self, event_type: EventType) -> usize {
		self.inner.listeners.read_sync(&event_type, |_, entries| entries.len()).unwrap_or(0) 
	}

	fn register<L: Listener + 'static>(
		&self,
		event_type: EventType,
		listener: L,
		once: bool,
	) -> Result<ListenerId, Error> {
		if !self.inner.accepting.load(Ordering::Acquire) {
			return Err(Error::NotRunning);
		}

		let listener = Arc::new(listener);
		let name = SmolStr::new(listener.name());
		let priority = listener.priority();
		let id = ListenerId(self.inner.next_id.fetch_add(1, Ordering::Relaxed));

		match self.inner.listeners.entry_sync(event_type) {
			scc::hash_map::Entry::Occupied(mut e) => {
				let entries = e.get_mut();
				if entries.iter().any(|e| e.name == name) {
					return Err(Error::AlreadyListening { event_type, listener: name });
				}
				entries.push(ListenerEntry { id, name, priority, listener, once });
			}
			scc::hash_map::Entry::Vacant(e) => {
				e.insert_entry(vec![ListenerEntry { id, name, priority, listener, once }]);
			}
		}

		self.inner.id_to_type.insert_sync(id, event_type).ok();
		Ok(id)
	}
}

impl Default for EventEmitter {
	fn default() -> Self {
		Self {
			inner: Arc::new(Inner {
				accepting: AtomicBool::new(false),
				listeners: scc::HashMap::new(),
				id_to_type: scc::HashMap::new(),
				next_id: AtomicU64::new(1),
			}),
		}
	}
}
