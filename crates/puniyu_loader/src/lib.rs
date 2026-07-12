//! # puniyu_loader
//!
//! 组件加载器 trait 定义，负责发现适配器和插件。

use async_trait::async_trait;
use puniyu_adapter_core::Adapter;
use puniyu_error::AnyError;
use puniyu_plugin_core::Plugin;
use std::sync::Arc;

/// 组件加载器。
#[async_trait]
pub trait Loader: Send + Sync + 'static {
	/// 加载器名称
	fn name(&self) -> &str;

	/// 发现适配器
	async fn adapters(&self) -> AnyError<Vec<Arc<dyn Adapter>>> {
		Ok(Vec::new())
	}

	/// 发现插件
	async fn plugins(&self) -> AnyError<Vec<Arc<dyn Plugin>>> {
		Ok(Vec::new())
	}
}
