use puniyu_common::source::SourceType;
use puniyu_error::registry::Error;
use puniyu_server_core::{ServerFunction, ServerId, ServerInfo};
use std::sync::LazyLock;

mod store;
use store::ServerStore;

static STORE: LazyLock<ServerStore> = LazyLock::new(ServerStore::new);

/// 服务器注册表
///
/// 用于注册和管理服务器配置。
///
/// # 注意
///
/// - 服务器配置使用 `Fn`，可以被多次调用
/// - `all()` 返回所有注册配置的克隆，不会清空注册表
#[derive(Debug, Default)]
pub struct ServerRegistry;

impl ServerRegistry {
	/// 注册服务器配置
	///
	/// # 参数
	///
	/// - `source` - 服务器来源
	/// - `server` - 服务器配置函数
	///
	/// # 返回
	///
	/// 成功返回服务器索引，失败返回错误
	pub fn register(source: SourceType, server: impl Into<ServerFunction>) -> Result<u64, Error> {
		let server = ServerInfo { source, builder: server.into() };
		STORE.insert(server)
	}

	/// 获取服务器配置
	///
	/// # 参数
	///
	/// - `source` - 服务器来源
	///
	/// # 返回
	///
	/// 返回服务器配置的克隆，如果不存在则返回 None
	pub fn get(source: SourceType) -> Option<ServerInfo> {
		let raw = STORE.raw();
		let map = raw.read().expect("Failed to acquire lock");

		map.values().find(|server| server.source == source).cloned()
	}

	/// 获取所有服务器配置
	///
	/// # 返回
	///
	/// 返回所有注册的服务器配置的克隆
	pub fn all() -> Vec<ServerInfo> {
		STORE.all()
	}

	/// 注销服务器
	///
	/// # 参数
	///
	/// - `server` - 服务器标识符（索引或来源）
	pub fn unregister<S>(server: S) -> Result<(), Error>
	where
		S: Into<ServerId>,
	{
		let server = server.into();
		match server {
			ServerId::Index(index) => Self::unregister_with_index(index),
			ServerId::Source(source) => Self::unregister_with_source(source),
		}
	}

	/// 通过索引注销服务器
	pub fn unregister_with_index(index: u64) -> Result<(), Error> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		if map.get(&index).is_none() {
			return Err(Error::NotFound("Server".to_string()));
		}
		map.remove(&index);
		Ok(())
	}

	/// 通过来源注销服务器
	pub fn unregister_with_source(source: SourceType) -> Result<(), Error> {
		let raw = STORE.raw();
		let mut map = raw.write().expect("Failed to acquire lock");
		let count = map.len();
		map.retain(|_, server| server.source != source);

		if map.len() == count {
			return Err(Error::NotFound("Server".to_string()));
		}
		Ok(())
	}
}
