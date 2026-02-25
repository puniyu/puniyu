//! # puniyu_dispatch
//!
//! 事件分发和处理库，提供全局事件总线用于异步事件分发。
//!
//! ## 功能特性
//!
//! - 全局事件总线，支持跨线程事件分发
//! - 异步事件处理，使用 tokio 异步运行时
//! - 自动事件路由到注册的处理器
//! - 优雅关闭机制
//!
//! ## 快速开始
//!
//! ### 初始化事件总线
//!
//! ```rust,ignore
//! use puniyu_dispatch::{EventBus, setup_event_bus};
//!
//! // 创建并设置全局事件总线
//! let event_bus = EventBus::new();
//! setup_event_bus(event_bus);
//!
//! // 启动事件处理循环
//! let event_bus = puniyu_dispatch::get_event_bus();
//! event_bus.run();
//! ```
//!
//! ### 发送事件
//!
//! ```rust,ignore
//! use std::sync::Arc;
//! use puniyu_dispatch::get_event_bus;
//! use puniyu_event::Event;
//!
//! // 获取事件总线
//! let event_bus = get_event_bus();
//!
//! // 发送事件（事件必须用 Arc 包装）
//! let event = Arc::new(my_event);
//! event_bus.send_event(event);
//! ```
//!
//! ## 工作原理
//!
//! 1. 事件通过 `send_event()` 发送到事件总线
//! 2. 事件总线使用异步通道接收事件
//! 3. 事件被分发到所有注册的处理器
//! 4. 处理器按优先级（rank）顺序执行

use puniyu_event::Event;
use puniyu_handler::HandlerRegistry;
use puniyu_logger::owo_colors::OwoColorize;
use puniyu_logger::{error, warn};
use std::sync::{Arc, Mutex, OnceLock};
use tokio::runtime::Handle;
use tokio::sync::{broadcast, mpsc};
use tokio::task::JoinHandle;

/// 事件发送器类型
///
/// 使用 `Arc` 包装事件以实现跨线程共享和避免生命周期问题
pub type EventSender = mpsc::Sender<Arc<Event<'static>>>;

/// 事件接收器类型
pub type EventReceiver = mpsc::Receiver<Arc<Event<'static>>>;

/// 全局事件总线实例
pub static EVENT_BUS: OnceLock<EventBus> = OnceLock::new();

/// 事件总线
///
/// 负责事件的分发和处理，使用异步通道进行事件传递。
#[derive(Debug)]
pub struct EventBus {
	/// 事件发送器
	sender: EventSender,
	/// 事件接收器（包装在 Mutex 中以支持单次获取）
	receiver: Arc<Mutex<Option<EventReceiver>>>,
	/// 关闭信号发送器
	shutdown_tx: broadcast::Sender<()>,
	/// Tokio 运行时句柄
	handle: Handle,
}

impl EventBus {
	/// 创建新的事件总线实例
	///
	/// 默认通道容量为 5000 个事件。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let event_bus = EventBus::new();
	/// ```
	pub fn new() -> Self {
		Self::with_capacity(5000)
	}

	/// 创建指定容量的事件总线实例
	///
	/// # 参数
	///
	/// - `capacity`: 事件通道的容量
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let event_bus = EventBus::with_capacity(10000);
	/// ```
	pub fn with_capacity(capacity: usize) -> Self {
		let (sender, receiver) = mpsc::channel(capacity);
		let (shutdown_tx, _) = broadcast::channel(1);
		Self {
			sender,
			receiver: Arc::new(Mutex::new(Some(receiver))),
			shutdown_tx,
			handle: Handle::current(),
		}
	}

	/// 启动事件总线的事件处理循环
	///
	/// 此方法会启动一个异步任务，持续监听事件通道并分发事件到处理器。
	/// 事件处理循环会一直运行，直到调用 `shutdown()` 或事件通道关闭。
	///
	/// # 返回值
	///
	/// 返回事件处理任务的 `JoinHandle`，可用于等待任务完成
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let event_bus = get_event_bus();
	/// let handle = event_bus.run();
	///
	/// // 等待事件总线关闭
	/// handle.await.unwrap();
	/// ```
	pub fn run(&self) -> JoinHandle<()> {
		let receiver = self.receiver.clone();
		let mut shutdown_rx = self.shutdown_tx.subscribe();

		self.handle.spawn(async move {
			let receiver = {
				let mut guard = receiver.lock().expect("Failed to acquire lock");
				guard.take()
			};
			let Some(mut receiver) = receiver else {
				return;
			};

			loop {
				tokio::select! {
					_ = shutdown_rx.recv() => {
						warn!("事件总线已关闭");
						break;
					}

					event_option = receiver.recv() => {
						match event_option {
							Some(event) => {
								Self::dispatch_event(&event).await;
							}
							None => {
								warn!("[{}]: 事件通道已关闭", "Event".blue());
								break;
							}
						}
					}
				}
			}
		})
	}

	/// 分发事件到所有注册的处理器
	///
	/// 处理器按优先级（rank）排序后依次执行。
	///
	/// # 参数
	///
	/// - `event`: 要分发的事件
	async fn dispatch_event(event: &Event<'_>) {
		let mut handlers = HandlerRegistry::all();
		handlers.sort_unstable_by_key(|a| a.rank());

		for handler in handlers {
			if let Err(e) = handler.handle(event).await {
				error!("[{}]: 处理器 {} 执行失败: {:?}", "Event".blue(), handler.name(), e);
			}
		}
	}

	/// 发送事件到事件总线
	///
	/// # 参数
	///
	/// - `event`: 要发送的事件（使用 `Arc` 包装）
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use std::sync::Arc;
	///
	/// let event = Arc::new(my_event);
	/// event_bus.send_event(event);
	/// ```
	pub fn send_event(&self, event: Arc<Event<'static>>) {
		let sender = self.sender.clone();
		self.handle.spawn(async move {
			if let Err(e) = sender.send(event).await {
				warn!("[{}]: 事件发送失败: {:?}", "Event".blue(), e);
			}
		});
	}

	/// 关闭事件总线
	///
	/// 发送关闭信号，停止事件处理循环。
	/// 此方法是非阻塞的，不会等待事件处理循环完全停止。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let event_bus = get_event_bus();
	/// event_bus.shutdown();
	/// ```
	pub fn shutdown(&self) {
		let _ = self.shutdown_tx.send(());
	}

	/// 获取事件发送器的克隆
	///
	/// 可用于在其他地方直接发送事件到通道。
	///
	/// # 返回值
	///
	/// 返回事件发送器的克隆
	pub fn sender(&self) -> EventSender {
		self.sender.clone()
	}

	/// 检查事件总线是否正在运行
	///
	/// # 返回值
	///
	/// 如果接收器已被取出（即 `run()` 已被调用），返回 `true`
	pub fn is_running(&self) -> bool {
		self.receiver.lock().expect("Failed to acquire lock").is_none()
	}
}

impl Default for EventBus {
	fn default() -> Self {
		Self::new()
	}
}

/// 设置全局事件总线
///
/// # 参数
///
/// - `event_bus`: 事件总线实例
///
/// # Panics
///
/// 如果事件总线已经初始化，则会 panic
///
/// # 示例
///
/// ```rust,ignore
/// let event_bus = EventBus::new();
/// setup_event_bus(event_bus);
/// ```
pub fn setup_event_bus(event_bus: EventBus) {
	EVENT_BUS.set(event_bus).expect("Failed to set event bus: EVENT_BUS is already initialized");
}

/// 获取全局事件总线的引用
///
/// # 返回值
///
/// 返回事件总线的引用
///
/// # Panics
///
/// 如果事件总线未初始化，则会 panic
///
/// # 示例
///
/// ```rust,ignore
/// let event_bus = get_event_bus();
/// event_bus.send_event(event);
/// ```
pub fn get_event_bus() -> &'static EventBus {
	EVENT_BUS.get().expect("Event bus is not initialized")
}
