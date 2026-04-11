//! # puniyu_runtime
//!
//! 统一的 puniyu 运行时 trait 定义。
//!
//! ## 特性
//!
//! - 提供 [`Runtime`] 作为只读运行时访问视图
//! - 提供 [`SendMessage`] 作为适配器发送能力 trait
//! - 提供 [`FrameworkRuntime`] 供框架内部组合 runtime 与发送能力
//! - 提供 [`Runtime::downcast_ref`] 访问适配器私有运行时能力
//!
//! ## 示例
//!
//! ```rust,ignore
//! use async_trait::async_trait;
//! use puniyu_runtime::{FrameworkRuntime, Runtime, SendMessage};
//! use puniyu_adapter_types::SendMsgType;
//! use puniyu_contact::ContactType;
//! use puniyu_message::Message;
//!
//! struct MyRuntime;
//!
//! #[async_trait]
//! impl SendMessage for MyRuntime {
//!     async fn send_message(
//!         &self,
//!         _contact: &ContactType<'_>,
//!         _message: &Message,
//!     ) -> puniyu_error::Result<SendMsgType> {
//!         Ok(SendMsgType { message_id: "msg-1".into(), time: 0 })
//!     }
//! }
//!
//! let runtime: &dyn Runtime = &MyRuntime;
//! let _ = runtime.downcast_ref::<MyRuntime>();
//!
//! let framework_runtime: &dyn FrameworkRuntime = &MyRuntime;
//! let _ = framework_runtime;
//! ```

use std::any::Any;

use async_trait::async_trait;
use puniyu_adapter_types::SendMsgType;
use puniyu_contact::ContactType;
use puniyu_error::Result;
use puniyu_message::Message;

pub trait Runtime: Any + Send + Sync {}

impl<T> Runtime for T where T: Any + Send + Sync {}

impl dyn Runtime {
	pub fn as_any(&self) -> &dyn Any {
		self
	}

	pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
		self.as_any().downcast_ref::<T>()
	}
}

#[async_trait]
pub trait SendMessage: Send + Sync {
	async fn send_message(
		&self,
		contact: &ContactType<'_>,
		message: &Message,
	) -> Result<SendMsgType>;
}

#[doc(hidden)]
#[async_trait]
pub trait FrameworkRuntime: Runtime + SendMessage {}

impl<T> FrameworkRuntime for T where T: Runtime + SendMessage {}
