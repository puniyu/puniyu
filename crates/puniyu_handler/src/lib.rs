//! # puniyu_handler
//!
//! 处理器库，提供统一的 `Handler` 接口。
//!
//! ## 特性
//!
//! - `Handler` trait 定义事件处理模型
//! - 支持处理 `puniyu_event::Event`
//! - 支持前置、后置和短路的洋葱调用链
//! - 支持优先级排序
//! - `Handler` trait 定义命令级处理器接口

use std::sync::Arc;

use async_trait::async_trait;

/// 命令处理器接口。
#[async_trait]
pub trait Handler: Send + Sync {
	/// 执行命令。
	async fn handle(
		&self,
		session: puniyu_session::MessageSession,
		params: puniyu_param::Params,
	) -> puniyu_error::AnyError;
}

#[async_trait]
impl<H: Handler + ?Sized> Handler for &H {
	async fn handle(
		&self,
		session: puniyu_session::MessageSession,
		params: puniyu_param::Params,
	) -> puniyu_error::AnyError {
		(**self).handle(session, params).await
	}
}

#[async_trait]
impl<H: Handler + ?Sized> Handler for Box<H> {
	async fn handle(
		&self,
		session: puniyu_session::MessageSession,
		params: puniyu_param::Params,
	) -> puniyu_error::AnyError {
		(**self).handle(session, params).await
	}
}

#[async_trait]
impl<H: Handler + ?Sized> Handler for Arc<H> {
	async fn handle(
		&self,
		session: puniyu_session::MessageSession,
		params: puniyu_param::Params,
	) -> puniyu_error::AnyError {
		(**self).handle(session, params).await
	}
}
