mod store;

use crate::error::Error;
use crate::registry::store::HandlerStore;
use crate::{Handler, HandlerId};
use std::sync::LazyLock;
use std::sync::atomic::{AtomicU64, Ordering};

static HANDLER_INDEX: AtomicU64 = AtomicU64::new(0);
static STORE: LazyLock<HandlerStore> = LazyLock::new(HandlerStore::new);

/// 处理器注册表。
pub struct HandlerRegistry;

impl<'h> HandlerRegistry {
	/// 注册处理器。
	pub fn register(handler: impl Into<std::sync::Arc<dyn Handler>>) -> Result<u64, Error> {
		let handler = handler.into();
		let mut map = STORE.raw().write().expect("poisoned lock");
		if map.values().any(|v| v.name() == handler.name()) {
			return Err(Error::Exists(handler.name().to_string()));
		}
		let index = HANDLER_INDEX.fetch_add(1, Ordering::Relaxed);
		map.insert(index, handler);
		Ok(index)
	}

	/// 卸载处理器（按索引或名称）。
	pub fn unregister<H>(handler: H) -> Result<(), Error>
	where
		H: Into<HandlerId<'h>>,
	{
		let handler = handler.into();
		match handler {
			HandlerId::Index(index) => Self::unregister_with_index(index),
			HandlerId::Name(name) => Self::unregister_with_handler_name(name.as_ref()),
		}
	}

	/// 按索引卸载处理器。
	pub fn unregister_with_index(index: u64) -> Result<(), Error> {
		let mut map = STORE.raw().write().expect("poisoned lock");
		match map.remove(&index) {
			Some(_) => Ok(()),
			None => Err(Error::NotFound(format!("index {index}"))),
		}
	}

	/// 按处理器名称卸载处理器。
	pub fn unregister_with_handler_name(name: &str) -> Result<(), Error> {
		let mut map = STORE.raw().write().expect("poisoned lock");
		let key = map.iter().find(|(_, h)| h.name() == name).map(|(k, _)| *k);
		match key.and_then(|k| map.remove(&k)) {
			Some(_) => Ok(()),
			None => Err(Error::NotFound(format!("name \"{name}\""))),
		}
	}

	/// 获取处理器（按索引或名称）。
	pub fn get<H>(handler: H) -> Option<std::sync::Arc<dyn Handler>>
	where
		H: Into<HandlerId<'h>>,
	{
		let handler = handler.into();
		match handler {
			HandlerId::Index(index) => Self::get_with_index(index),
			HandlerId::Name(name) => Self::get_with_handler_name(name.as_ref()),
		}
	}

	/// 按索引获取处理器。
	pub fn get_with_index(index: u64) -> Option<std::sync::Arc<dyn Handler>> {
		let map = STORE.raw().read().expect("poisoned lock");
		map.get(&index).cloned()
	}

	/// 按名称获取处理器。
	pub fn get_with_handler_name(name: &str) -> Option<std::sync::Arc<dyn Handler>> {
		let map = STORE.raw().read().expect("poisoned lock");
		map.values().find(|handler| handler.name() == name).cloned()
	}

	/// 获取所有处理器。
	pub fn all() -> Vec<std::sync::Arc<dyn Handler>> {
		STORE.raw().read().expect("poisoned lock").values().cloned().collect()
	}
}
