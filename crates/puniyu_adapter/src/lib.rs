//! # puniyu_adapter
//!
//! 适配器库，提供适配器的统一接口定义和注册管理。
//!
//! ## 概述
//!
//! `puniyu_adapter` 是 Puniyu 框架的适配器库，定义了适配器的统一接口。
//! 该库为不同平台的适配器实现提供了标准化的 trait 定义。
//!
//! ## 使用方式
//!
//! ### 实现适配器
//!
//! ```rust,ignore
//! use puniyu_adapter::Adapter;
//! use puniyu_adapter::api::AdapterApi;
//! use puniyu_adapter::types::info::AdapterInfo;
//!
//! struct MyAdapter {
//!     info: AdapterInfo,
//!     api: AdapterApi,
//! }
//!
//! #[async_trait::async_trait]
//! impl Adapter for MyAdapter {
//!     fn info(&self) -> &AdapterInfo {
//!         &self.info
//!     }
//!
//!     fn api(&self) -> &AdapterApi {
//!         &self.api
//!     }
//!
//!     async fn init(&self) -> puniyu_error::Result {
//!         // 初始化逻辑
//!         Ok(())
//!     }
//! }
//! ```
//!
//! ### 使用适配器
//!
//! ```rust,ignore
//! use puniyu_adapter::Adapter;
//!
//! async fn use_adapter(adapter: &dyn Adapter) {
//!     // 初始化适配器
//!     adapter.init().await?;
//!
//!     // 获取 API
//!     let api = adapter.api();
//!     api.message().send_msg(&contact, &message).await?;
//! }
//! ```

#[cfg(feature = "registry")]
mod registry;
#[cfg(feature = "registry")]
pub use registry::AdapterRegistry;
pub mod types;
use crate::api::AdapterApi;
#[doc(inline)]
pub use puniyu_adapter_core::adapter_info;
#[doc(inline)]
pub use puniyu_adapter_core::api;
use puniyu_config::types::Config;
use puniyu_hook::Hook;
use std::sync::Arc;
use types::AdapterInfo;

/// 适配器 Trait
///
/// 定义了适配器的基本行为和接口。所有适配器实现都必须实现此 trait。
///
/// # 示例
///
/// ```rust,ignore
/// use puniyu_adapter::Adapter;
/// use puniyu_adapter::api::AdapterApi;
/// use puniyu_adapter::types::info::AdapterInfo;
///
/// struct MyAdapter {
///     info: AdapterInfo,
///     api: AdapterApi,
/// }
///
/// #[async_trait::async_trait]
/// impl Adapter for MyAdapter {
///     fn info(&self) -> &AdapterInfo {
///         &self.info
///     }
///
///     fn api(&self) -> &AdapterApi {
///         &self.api
///     }
/// }
/// ```


