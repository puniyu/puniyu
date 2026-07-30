//! # puniyu_handler
//!
//! 处理器库，提供统一的 `Handler` 接口。

use std::sync::Arc;

use async_trait::async_trait;

/// 处理器接口。
///
/// 接收 [`EventSession`](puniyu_session::EventSession)。
/// Handler 内部自行完成数据提取和处理。
#[async_trait]
pub trait Handler: Send + Sync {
	/// 执行处理逻辑。
	async fn handle(
		&self,
		session: puniyu_session::EventSession,
	) -> puniyu_error::AnyError;
}

macro_rules! impl_handler_deref {
	($wrapper:ty) => {
		#[async_trait]
		impl<H: Handler + ?Sized> Handler for $wrapper {
			async fn handle(
				&self,
				session: puniyu_session::EventSession,
			) -> puniyu_error::AnyError {
				(**self).handle(session).await
			}
		}
	};
}

impl_handler_deref!(&H);
impl_handler_deref!(Box<H>);
impl_handler_deref!(Arc<H>);
