//! # puniyu_adapter_runtime
//!
//! 统一的 puniyu 适配器 API 库。
//!
//! 当前版本不再聚合固定的 message/group/friend/account 子 API，而是将
//! [`AdapterRuntime`] 设计为具体适配器 runtime 的轻量包装层。
//!
//! ## 特性
//!
//! - 提供 [`AdapterRuntime`] 统一封装适配器 runtime
//! - 提供 [`Runtime`] 作为跨适配器的最小运行时抽象
//! - 提供 [`AdapterRuntime::send_message`] 统一消息发送入口
//! - 提供 [`AdapterRuntime::runtime`] 下转型访问适配器私有能力
//!
//! ## 示例
//!
//! ```rust,ignore
//! use std::{any::Any, sync::Arc};
//!
//! use async_trait::async_trait;
//! use puniyu_adapter_runtime::{AdapterRuntime, Error, Runtime};
//! use puniyu_adapter_types::SendMsgType;
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
//!     ) -> Result<SendMsgType, Error> {
//!         Ok(SendMsgType { message_id: "msg-1".into(), time: 0 })
//!     }
//!
//! }
//!
//! let runtime = AdapterRuntime::from_runtime(MyRuntime);
//! let _ = runtime.runtime::<MyRuntime>();
//! ```

mod error;
#[doc(inline)]
pub use error::Error;
mod runtime;
#[doc(inline)]
pub use runtime::Runtime;

use puniyu_adapter_types::SendMsgType;
use puniyu_contact::ContactType;
use puniyu_message::Message;
use std::{any::Any, sync::Arc};

#[derive(Clone)]
pub struct AdapterRuntime {
	inner: Arc<dyn Runtime>,
}

impl AdapterRuntime {
	pub fn new<R>(runtime: Arc<R>) -> Self
	where
		R: Runtime + 'static,
	{
		Self { inner: runtime }
	}

	pub fn from_runtime<R>(runtime: R) -> Self
	where
		R: Runtime + 'static,
	{
		Self { inner: Arc::new(runtime) }
	}

	pub async fn send_message(
		&self,
		contact: &ContactType<'_>,
		message: &Message,
	) -> Result<SendMsgType, Error> {
		self.inner.send_message(contact, message).await
	}

	pub fn runtime<T>(&self) -> Option<&T>
	where
		T: Any + Send + Sync + 'static,
	{
		(self.inner.as_ref() as &dyn Any).downcast_ref::<T>()
	}
}

impl PartialEq for AdapterRuntime {
	fn eq(&self, other: &Self) -> bool {
		Arc::ptr_eq(&self.inner, &other.inner)
	}
}
