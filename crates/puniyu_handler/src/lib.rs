//! # puniyu_handler
//!
//! 处理器库，提供统一的 `Handler` 接口。

use std::sync::Arc;

use async_trait::async_trait;
use puniyu_param::ParamValue;

/// 处理器接口。
///
/// 接收 [`EventSession`](puniyu_session::EventSession) 和提取器产出的 [`ParamValue`]。
#[async_trait]
pub trait Handler: Send + Sync {
	/// 执行处理逻辑。
	async fn handle(
		&self,
		session: puniyu_session::EventSession,
		params: ParamValue,
	) -> puniyu_error::AnyError;
}

#[async_trait]
impl<H: Handler + ?Sized> Handler for &H {
	async fn handle(
		&self,
		session: puniyu_session::EventSession,
		params: ParamValue,
	) -> puniyu_error::AnyError {
		(**self).handle(session, params).await
	}
}

#[async_trait]
impl<H: Handler + ?Sized> Handler for Box<H> {
	async fn handle(
		&self,
		session: puniyu_session::EventSession,
		params: ParamValue,
	) -> puniyu_error::AnyError {
		(**self).handle(session, params).await
	}
}

#[async_trait]
impl<H: Handler + ?Sized> Handler for Arc<H> {
	async fn handle(
		&self,
		session: puniyu_session::EventSession,
		params: ParamValue,
	) -> puniyu_error::AnyError {
		(**self).handle(session, params).await
	}
}
