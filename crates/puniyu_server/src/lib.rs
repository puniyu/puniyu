//! # puniyu_server
//!
//! Puniyu HTTP 服务器库，提供基于 Actix-Web 的 Web 服务功能。
//!
//! ## 概述
//!
//! `puniyu_server` 是 Puniyu 框架的 HTTP 服务器实现，提供了完整的 Web 服务功能，
//! 包括 API 路由、中间件、服务器控制和响应格式化等。
//!
//! ## 特性
//!
//! - 🚀 **易用性** - 简单的 API 启动和控制服务器
//! - 🔧 **可配置** - 支持自定义配置和中间件
//! - 🔄 **热重启** - 支持优雅关闭和重启服务器
//! - 📦 **模块化** - 基于 Actix-Web 的模块化架构
//! - 🎨 **可扩展** - 支持服务器注册表动态添加路由
//! - 📝 **标准响应** - 统一的 JSON 响应格式
//!
//! ## 快速开始
//!
//! ### 启动服务器
//!
//! ```rust,no_run
//! use puniyu_server::run_server_spawn;
//! use std::net::IpAddr;
//!
//! #[tokio::main]
//! async fn main() {
//!     let host = "127.0.0.1".parse::<IpAddr>().unwrap();
//!     let port = 8080;
//!     
//!     // 在新任务中启动服务器
//!     run_server_spawn(host, port);
//!     
//!     // 保持主线程运行
//!     tokio::signal::ctrl_c().await.ok();
//! }
//! ```
//!
//! ### 控制服务器
//!
//! ```rust,ignore
//! use puniyu_server::{stop_server, restart_server};
//! use std::net::IpAddr;
//!
//! // 停止服务器
//! stop_server().await;
//!
//! // 重启服务器
//! let host = "127.0.0.1".parse::<IpAddr>().unwrap();
//! restart_server(host, 8080).await;
//! ```
//!
//! ### 设置自定义 Logo
//!
//! ```rust,ignore
//! use puniyu_server::{set_logo, load_logo_from_file};
//! use bytes::Bytes;
//!
//! // 方式 1：直接设置
//! let logo_data = Bytes::from(vec![/* logo data */]);
//! set_logo(logo_data);
//!
//! // 方式 2：从文件加载
//! load_logo_from_file("path/to/logo.png").ok();
//! ```
//!
//! ## 主要功能
//!
//! ### 服务器控制
//!
//! - `run_server(host, port)` - 运行服务器（阻塞）
//! - `run_server_spawn(host, port)` - 在新任务中启动服务器
//! - `stop_server()` - 优雅关闭服务器
//! - `restart_server(host, port)` - 重启服务器
//!
//! ### Logo 管理
//!
//! - `set_logo(data)` - 设置 Logo 图片
//! - `load_logo_from_file(path)` - 从文件加载 Logo
//!
//! ### 服务器注册表（需要 `registry` 特性）
//!
//! ```rust,ignore
//! use puniyu_server::ServerRegistry;
//! use puniyu_server_core::ServerInfo;
//!
//! // 注册服务器配置
//! ServerRegistry::register(server_info);
//!
//! // 获取所有注册的服务器
//! let servers = ServerRegistry::all();
//! ```
//!
//! ## 内置路由
//!
//! 服务器提供以下内置路由：
//!
//! - `GET /` - 欢迎页面
//! - `GET /logo` - Logo 图片
//! - `GET /logo.png` - Logo 图片（别名）
//! - `GET /api/v1/*` - API 路由
//!
//! ## 特性标志
//!
//! - `cli` - 启用命令行界面支持（默认启用）
//! - `registry` - 启用服务器注册表功能
//!
//! ## 配置
//!
//! 服务器配置通过 `puniyu_config` 管理，支持：
//!
//! - 服务器地址和端口
//! - 日志级别
//! - 中间件配置
//! - 自定义路由
//!
//! ## 中间件
//!
//! 服务器内置以下中间件：
//!
//! - **访问日志** - 记录所有 HTTP 请求
//! - **路径规范化** - 自动处理尾部斜杠
//!
//! ## 示例
//!
//! ### 完整的服务器应用
//!
//! ```rust,no_run
//! use puniyu_server::{run_server, set_logo, load_logo_from_file};
//! use std::net::IpAddr;
//!
//! #[tokio::main]
//! async fn main() -> std::io::Result<()> {
//!     // 加载 Logo
//!     load_logo_from_file("resources/logo.png").ok();
//!     
//!     // 启动服务器
//!     let host = "0.0.0.0".parse::<IpAddr>().unwrap();
//!     run_server(host, 8080).await
//! }
//! ```

mod api;
mod config;
mod logger;
mod middleware;
#[cfg(feature = "registry")]
mod registry;
#[cfg(feature = "registry")]
pub use registry::ServerRegistry;
mod server;
pub use server::{restart_server, run_server, run_server_spawn, stop_server};
mod response;
pub(crate) use response::BaseResponse;

use bytes::Bytes;
use std::sync::{Arc, LazyLock, RwLock};

/// 自定义 Logo 图片数据
///
/// 可以通过 `set_logo()` 函数设置或更新 Logo。
pub(crate) static LOGO: LazyLock<Arc<RwLock<Option<Bytes>>>> =
	LazyLock::new(|| Arc::new(RwLock::new(None)));

/// 设置 Logo 图片
///
/// # 参数
///
/// - `data` - Logo 图片的二进制数据
///
/// # 示例
///
/// ```rust,ignore
/// use bytes::Bytes;
///
/// let logo_data = Bytes::from(vec![/* logo data */]);
/// set_logo(logo_data);
/// ```
pub fn set_logo(data: Bytes) {
	if let Ok(mut logo) = LOGO.write() {
		*logo = Some(data);
	}
}

/// 从文件加载 Logo
///
/// # 参数
///
/// - `path` - Logo 文件路径
///
/// # 返回
///
/// 成功返回 `Ok(())`，失败返回 `std::io::Error`
///
/// # 示例
///
/// ```rust,ignore
/// load_logo_from_file("path/to/logo.png").ok();
/// ```
pub fn load_logo_from_file(path: impl AsRef<std::path::Path>) -> std::io::Result<()> {
	let data = std::fs::read(path)?;
	set_logo(Bytes::from(data));
	Ok(())
}
