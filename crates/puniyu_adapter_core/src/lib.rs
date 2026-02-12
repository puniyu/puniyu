//! # puniyu_adapter_core
//!
//! 适配器核心库，提供适配器 API 接口和类型定义。
//!
//! ## 概述
//!
//! `puniyu_adapter_core` 定义了 Puniyu 框架中适配器的标准接口和类型系统。
//! 该库为不同平台的适配器提供统一的抽象层，包括消息、群组、好友、账户等核心功能。
//!
//! ## 主要模块
//!
//! - [`api`] - 适配器 API 接口定义
//!   - [`api::MessageApi`] - 消息相关 API
//!   - [`api::GroupApi`] - 群组相关 API
//!   - [`api::FriendApi`] - 好友相关 API
//!   - [`api::AccountApi`] - 账户相关 API
//! - [`types`] - 类型定义
//!   - [`types::AdapterInfo`] - 适配器信息
//!   - [`types::MessageType`] - 消息类型
//!   - [`types::UserInfo`] - 用户信息
//!   - [`types::GroupInfo`] - 群组信息
//!
//! ## 快速开始
//!
//! ### 实现适配器
//!
//! ```rust,ignore
//! use puniyu_adapter_core::{Adapter, AdapterApi, AdapterInfo};
//! use async_trait::async_trait;
//!
//! struct MyAdapter {
//!     info: AdapterInfo,
//!     api: AdapterApi,
//! }
//!
//! #[async_trait]
//! impl Adapter for MyAdapter {
//!     type Context = ();
//!
//!     fn info(&self) -> &AdapterInfo {
//!         &self.info
//!     }
//!
//!     fn api(&self) -> &AdapterApi {
//!         &self.api
//!     }
//! }
//! ```
//!
//! ### 使用 API
//!
//! ```rust,ignore
//! use puniyu_adapter_core::api::MessageApi;
//!
//! async fn send_message(api: &impl MessageApi) {
//!     // 发送消息
//!     // api.send_msg(...).await?;
//! }
//! ```

pub mod api;
pub mod types;
