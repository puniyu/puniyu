//! # puniyu_adapter_core
//!
//! 统一的 puniyu 适配器核心库，覆盖适配器定义、元信息与注册表管理场景。
//!
//! ## 特性
//!
//! - 提供 [`Adapter`] trait 定义适配器行为
//! - 提供 [`AdapterRegistry`] 管理适配器注册与查询
//! - 组合 `puniyu_adapter_runtime` 与 `puniyu_adapter_types`
//! - 支持配置、钩子、服务器与初始化流程扩展
//!
//! ## 示例
//!
//! ```rust,ignore
//! use std::{any::Any, sync::Arc};
//!
//! use async_trait::async_trait;
//! use puniyu_adapter_runtime::{AdapterRuntime, Runtime};
//! use puniyu_adapter_core::Adapter;
//! use puniyu_adapter_types::{adapter_info, AdapterPlatform, AdapterProtocol, SendMsgType};
//! use puniyu_contact::ContactType;
//! use puniyu_message::Message;
//!
//! struct MyRuntime;
//!
//! #[async_trait]
//! impl Runtime for MyRuntime {
//!     async fn send_message(
//!         &self,
//!         _contact: &ContactType<'_>,
//!         _message: &Message,
//!     ) -> puniyu_error::Result<SendMsgType> {
//!         Ok(SendMsgType { message_id: "msg-1".into(), time: 0 })
//!     }
//!
//! }
//!
//! struct MyAdapter;
//!
//! #[async_trait]
//! impl Adapter for MyAdapter {
//!     fn info(&self) -> puniyu_adapter_types::AdapterInfo {
//!         adapter_info!("console", AdapterPlatform::QQ, AdapterProtocol::Console)
//!     }
//!
//!     fn runtime(&self) -> AdapterRuntime {
//!         AdapterRuntime::from_runtime(MyRuntime)
//!     }
//! }
//! ```

mod registry;
#[doc(inline)]
pub use registry::AdapterRegistry;
mod types;
#[doc(inline)]
pub use types::*;

use puniyu_adapter_runtime::AdapterRuntime;
use puniyu_adapter_types::AdapterInfo;
use puniyu_config::Config;
use puniyu_hook::Hook;
use std::sync::Arc;

#[async_trait::async_trait]
pub trait Adapter: Send + Sync + 'static {
	/// 获取适配器信息。
	fn info(&self) -> AdapterInfo;

	/// 获取适配器运行时。
	fn runtime(&self) -> AdapterRuntime;

	/// 获取配置列表。
	fn config(&self) -> Vec<Arc<dyn Config>> {
		Vec::new()
	}

	/// 获取钩子列表。
	fn hook(&self) -> Vec<Arc<dyn Hook>> {
		Vec::new()
	}

	/// 获取服务器扩展。
	fn server(&self) -> Option<puniyu_server::ServerFunction> {
		None
	}

	/// 初始化适配器。
	async fn init(&self) -> puniyu_error::Result {
		log::info!("Adapter: 初始化完成");
		Ok(())
	}
}

impl PartialEq for dyn Adapter {
	fn eq(&self, other: &Self) -> bool {
		self.info() == other.info()
	}
}
