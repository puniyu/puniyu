//! # puniyu_server_core
//!
//! Puniyu 服务器核心库，提供服务器配置的基础类型定义。
//!
//! ## 概述
//!
//! `puniyu_server_core` 是 Puniyu 框架的服务器核心库，定义了服务器配置相关的核心类型。
//! 该库基于 Actix-Web 框架，为适配器和插件提供统一的服务器配置接口。
//!
//! ## 特性
//!
//! - 🎯 **类型安全** - 使用类型别名和结构体确保服务器配置的一致性
//! - 🔧 **简洁接口** - 提供简单直观的配置函数类型
//! - 🔄 **线程安全** - 支持 Send + Sync，可在多线程环境中使用
//! - 📦 **轻量级** - 最小化依赖，仅依赖 Actix-Web 和 puniyu_common
//! - 🏷️ **来源追踪** - 通过 `ServerInfo` 追踪服务器配置的来源
//! - 🔁 **高效克隆** - 使用 `Arc` 包装，支持高效的配置共享
//!
//! ## 核心类型
//!
//! ### ServerFunction
//!
//! 服务器配置函数类型别名，用于配置 Actix-Web 服务。使用 `Fn` 表示可以多次调用，使用 `Arc` 包装以支持传递和共享：
//!
//! ```rust,ignore
//! pub type ServerFunction = Arc<dyn Fn(&mut ServiceConfig) + Send + Sync>;
//! ```
//!
//! ### ServerInfo
//!
//! 服务器信息结构体，包含来源和配置构建器。实现了 `Clone`，可以高效复制：
//!
//! ```rust,ignore
//! #[derive(Clone)]
//! pub struct ServerInfo {
//!     pub source: SourceType,
//!     pub builder: ServerFunction,
//! }
//! ```
//!
//! ### ServerId
//!
//! 服务器标识符枚举，可通过索引或来源类型标识服务器：
//!
//! ```rust,ignore
//! pub enum ServerId {
//!     Index(u64),
//!     Source(SourceType),
//! }
//! ```
//!
//! ## 快速开始
//!
//! ### 定义服务器配置函数
//!
//! ```rust
//! use puniyu_server_core::ServerFunction;
//! use actix_web::web::{ServiceConfig, resource};
//! use actix_web::HttpResponse;
//! use std::sync::Arc;
//!
//! fn create_server_config() -> ServerFunction {
//!     Arc::new(|cfg: &mut ServiceConfig| {
//!         cfg.service(
//!             resource("/health").to(|| async { HttpResponse::Ok().body("OK") })
//!         );
//!     })
//! }
//! ```
//!
//! ### 创建服务器信息
//!
//! ```rust,ignore
//! use puniyu_server_core::{ServerInfo, ServerFunction};
//! use puniyu_common::source::SourceType;
//! use actix_web::web::{ServiceConfig, resource};
//! use actix_web::HttpResponse;
//! use std::sync::Arc;
//!
//! let server_info = ServerInfo {
//!     source: SourceType::Adapter("my_adapter".to_string()),
//!     builder: Arc::new(|cfg: &mut ServiceConfig| {
//!         cfg.service(
//!             resource("/api").to(|| async { HttpResponse::Ok().body("API") })
//!         );
//!     }),
//! };
//!
//! // 克隆服务器信息（高效，仅增加引用计数）
//! let cloned = server_info.clone();
//! ```
//!
//! ### 使用服务器标识符
//!
//! ```rust,ignore
//! use puniyu_server_core::ServerId;
//! use puniyu_common::source::SourceType;
//!
//! // 通过索引标识
//! let id1 = ServerId::from(0u64);
//!
//! // 通过来源标识
//! let source = SourceType::Plugin("my_plugin".to_string());
//! let id2 = ServerId::from(source);
//! ```
//!
//! ## 使用场景
//!
//! ### 1. 适配器路由配置
//!
//! 适配器可以使用 `ServerInfo` 来定义自己的 HTTP 路由并标识来源：
//!
//! ```rust,ignore
//! use puniyu_server_core::{ServerInfo, ServerFunction};
//! use puniyu_common::source::SourceType;
//! use actix_web::web::{ServiceConfig, resource};
//! use actix_web::HttpResponse;
//! use std::sync::Arc;
//!
//! fn adapter_server(adapter_name: &str) -> ServerInfo {
//!     ServerInfo {
//!         source: SourceType::Adapter(adapter_name.to_string()),
//!         builder: Arc::new(|cfg: &mut ServiceConfig| {
//!             cfg.service(
//!                 resource("/api/send").to(|| async {
//!                     HttpResponse::Ok().body("Message sent")
//!                 })
//!             );
//!             cfg.service(
//!                 resource("/api/recall").to(|| async {
//!                     HttpResponse::Ok().body("Message recalled")
//!                 })
//!             );
//!         }),
//!     }
//! }
//! ```
//!
//! ### 2. 插件 API 端点
//!
//! 插件可以注册自己的 API 端点并标识来源：
//!
//! ```rust,ignore
//! use puniyu_server_core::{ServerInfo, ServerFunction};
//! use puniyu_common::source::SourceType;
//! use actix_web::web::{ServiceConfig, scope, resource};
//! use actix_web::HttpResponse;
//! use std::sync::Arc;
//!
//! fn plugin_server(plugin_name: &str) -> ServerInfo {
//!     ServerInfo {
//!         source: SourceType::Plugin(plugin_name.to_string()),
//!         builder: Arc::new(|cfg: &mut ServiceConfig| {
//!             cfg.service(
//!                 scope("/plugin")
//!                     .service(resource("/status").to(|| async {
//!                         HttpResponse::Ok().body("Running")
//!                     }))
//!                     .service(resource("/config").to(|| async {
//!                         HttpResponse::Ok().body("Updated")
//!                     }))
//!             );
//!         }),
//!     }
//! }
//! ```
//!
//! ### 3. 服务器管理与共享
//!
//! 使用 `ServerId` 管理多个服务器实例，利用 `Clone` 高效共享配置：
//!
//! ```rust,ignore
//! use puniyu_server_core::{ServerInfo, ServerId};
//! use std::collections::HashMap;
//!
//! struct ServerRegistry {
//!     servers: Vec<ServerInfo>,
//!     index_map: HashMap<String, usize>,
//! }
//!
//! impl ServerRegistry {
//!     fn get(&self, id: &ServerId) -> Option<&ServerInfo> {
//!         match id {
//!             ServerId::Index(idx) => self.servers.get(*idx as usize),
//!             ServerId::Source(source) => {
//!                 self.servers.iter().find(|s| &s.source == source)
//!             }
//!         }
//!     }
//!     
//!     fn clone_server(&self, id: &ServerId) -> Option<ServerInfo> {
//!         self.get(id).cloned() // 高效克隆，仅增加 Arc 引用计数
//!     }
//! }
//! ```

use actix_web::web::ServiceConfig;
use puniyu_common::source::SourceType;
use std::fmt::Debug;
use std::sync::Arc;

/// 服务器配置函数类型
///
/// 这是一个类型别名，表示一个可以配置 Actix-Web 服务的函数。
/// 使用 `Fn` 表示该函数可以被多次调用，使用 `Arc` 包装以支持高效的克隆和共享。
///
/// # 类型定义
///
/// ```rust,ignore
/// pub type ServerFunction = Arc<dyn Fn(&mut ServiceConfig) + Send + Sync>;
/// ```
///
/// # 特性
///
/// - 接受 `&mut ServiceConfig` 参数，用于配置服务
/// - 实现 `Send + Sync`，可在多线程环境中安全使用
/// - 使用 `Arc` 包装，支持动态分发和高效克隆
/// - 可以在多个地方共享和调用同一个配置函数
///
///
/// # Arc 的作用
///
/// - **延迟调用** - 可以在创建后的任意时刻调用
/// - **传递所有权** - 可以将配置函数传递给其他组件
/// - **高效克隆** - 克隆时仅增加引用计数，不复制函数本身
/// - **线程安全** - 可以在多个线程间安全共享
///
/// # 示例
///
/// ## 创建简单的路由配置
///
/// ```rust
/// use puniyu_server_core::ServerFunction;
/// use actix_web::web::{ServiceConfig, resource};
/// use actix_web::HttpResponse;
/// use std::sync::Arc;
///
/// let config: ServerFunction = Arc::new(|cfg: &mut ServiceConfig| {
///     cfg.service(
///         resource("/hello").to(|| async { HttpResponse::Ok().body("Hello!") })
///     );
/// });
/// ```
///
/// ## 捕获变量
///
/// ```rust,ignore
/// use puniyu_server_core::ServerFunction;
/// use actix_web::web::{ServiceConfig, resource};
/// use actix_web::HttpResponse;
/// use std::sync::Arc;
///
/// let routes = vec!["/api/users", "/api/posts"];
/// let config: ServerFunction = Arc::new(move |cfg: &mut ServiceConfig| {
///     for route in &routes {
///         cfg.service(
///             resource(route).to(|| async { HttpResponse::Ok() })
///         );
///     }
/// });
/// ```
///
/// ## 配置多个路由
///
/// ```rust,ignore
/// use puniyu_server_core::ServerFunction;
/// use actix_web::web::{ServiceConfig, scope, resource};
/// use actix_web::HttpResponse;
/// use std::sync::Arc;
///
/// let config: ServerFunction = Arc::new(|cfg: &mut ServiceConfig| {
///     cfg.service(
///         scope("/api")
///             .service(resource("/users").to(|| async {
///                 HttpResponse::Ok().body("Users")
///             }))
///     );
/// });
/// ```
///
/// ## 在结构体中使用
///
/// ```rust,ignore
/// use puniyu_server_core::ServerFunction;
/// use actix_web::web::{ServiceConfig, resource};
/// use actix_web::HttpResponse;
/// use std::sync::Arc;
///
/// #[derive(Clone)]
/// struct Server {
///     config: ServerFunction,
/// }
///
/// impl Server {
///     fn new(config: ServerFunction) -> Self {
///         Self { config }
///     }
///
///     fn apply_config(&self, cfg: &mut ServiceConfig) {
///         // 直接调用，无需 try_unwrap
///         (self.config)(cfg);
///     }
/// }
///
/// let config = Arc::new(|cfg: &mut ServiceConfig| {
///     cfg.service(resource("/test").to(|| async { HttpResponse::Ok() }));
/// });
/// let server = Server::new(config);
/// ```
///
/// # 注意事项
///
/// - 虽然可以多次调用，但通常服务器配置只在启动时执行一次
/// - 如果需要确保只调用一次，应在应用层面进行控制
/// - 捕获的变量应该是可以多次借用的（使用 `&` 或 `Clone`）
pub type ServerFunction = Arc<dyn Fn(&mut ServiceConfig) + Send + Sync>;

/// 服务器信息
///
/// 包含服务器的来源信息和配置构建器。用于标识和配置服务器实例。
///
/// # 字段
///
/// - `source` - 服务器来源类型，标识服务器的来源（适配器、插件等）
/// - `builder` - 服务器配置函数，用于配置 Actix-Web 服务
///
/// # 特性
///
/// - **可克隆** - 实现了 `Clone`，可以轻松复制服务器信息
/// - **来源追踪** - 通过 `source` 字段追踪服务器配置的来源
/// - **高效共享** - `builder` 使用 `Arc` 包装，克隆时仅增加引用计数
///
/// # 示例
///
/// ## 创建服务器信息
///
/// ```rust,ignore
/// use puniyu_server_core::{ServerInfo, ServerFunction};
/// use puniyu_common::source::SourceType;
/// use actix_web::web::{ServiceConfig, get, HttpResponse};
/// use std::sync::Arc;
///
/// let server_info = ServerInfo {
///     source: SourceType::Adapter("my_adapter".to_string()),
///     builder: Arc::new(|cfg: &mut ServiceConfig| {
///         cfg.route("/api", get(|| async { HttpResponse::Ok().body("API") }));
///     }),
/// };
/// ```
///
/// ## 克隆服务器信息
///
/// ```rust,ignore
/// use puniyu_server_core::ServerInfo;
///
/// let server_info = ServerInfo { /* ... */ };
/// let cloned = server_info.clone(); // 高效克隆，仅增加 Arc 引用计数
/// ```
///
/// ## 在集合中使用
///
/// ```rust,ignore
/// use puniyu_server_core::ServerInfo;
/// use std::collections::HashMap;
///
/// let mut servers: HashMap<String, ServerInfo> = HashMap::new();
/// servers.insert("adapter1".to_string(), server_info.clone());
/// ```
///
/// # 相等性比较
///
/// `ServerInfo` 实现了 `PartialEq`，仅比较 `source` 字段。
/// 这意味着具有相同来源的服务器被视为相等，即使它们的配置函数不同。
///
/// ```rust,ignore
/// let server1 = ServerInfo { source: source1, builder: builder1 };
/// let server2 = ServerInfo { source: source1, builder: builder2 };
/// assert_eq!(server1, server2); // 相同的 source，视为相等
/// ```
///
/// # Debug 输出
///
/// `ServerInfo` 实现了 `Debug`，仅输出 `source` 字段，
/// 因为 `ServerFunction` 无法被格式化输出。
///
/// ```rust,ignore
/// let server_info = ServerInfo { /* ... */ };
/// println!("{:?}", server_info); // 输出: ServerInfo { source: ... }
/// ```
#[derive(Clone)]
pub struct ServerInfo {
	/// 服务器来源类型
	pub source: SourceType,
	/// 服务器配置构建器
	pub builder: ServerFunction,
}

impl PartialEq for ServerInfo {
	fn eq(&self, other: &Self) -> bool {
		self.source == other.source
	}
}

impl Debug for ServerInfo {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ServerInfo").field("source", &self.source).finish()
	}
}
unsafe impl Send for ServerInfo {}
unsafe impl Sync for ServerInfo {}
/// 服务器标识符
///
/// 用于标识服务器的枚举类型，可以通过索引或来源类型来标识服务器。
///
/// # 变体
///
/// - `Index(u64)` - 通过数字索引标识服务器
/// - `Source(SourceType)` - 通过来源类型标识服务器
///
/// # 示例
///
/// ## 使用索引创建
///
/// ```rust
/// use puniyu_server_core::ServerId;
///
/// let id = ServerId::from(0u64);
/// // 或者直接使用
/// let id = ServerId::Index(0);
/// ```
///
/// ## 使用来源类型创建
///
/// ```rust,ignore
/// use puniyu_server_core::ServerId;
/// use puniyu_common::source::SourceType;
///
/// let source = SourceType::Adapter("my_adapter".to_string());
/// let id = ServerId::from(source);
/// // 或者直接使用
/// let id = ServerId::Source(source);
/// ```
///
/// # 类型转换
///
/// `ServerId` 实现了 `From<u64>` 和 `From<SourceType>`，
/// 可以方便地从这些类型转换：
///
/// ```rust,ignore
/// use puniyu_server_core::ServerId;
/// use puniyu_common::source::SourceType;
///
/// // 从 u64 转换
/// let id1: ServerId = 42u64.into();
///
/// // 从 SourceType 转换
/// let source = SourceType::Plugin("my_plugin".to_string());
/// let id2: ServerId = source.into();
/// ```
///
/// # 使用场景
///
/// `ServerId` 通常用于：
///
/// - 在服务器注册表中查找服务器
/// - 标识和管理多个服务器实例
/// - 根据来源或索引选择特定的服务器配置
pub enum ServerId {
	/// 通过索引标识服务器
	Index(u64),
	/// 通过来源类型标识服务器
	Source(SourceType),
}

impl From<u64> for ServerId {
	fn from(index: u64) -> Self {
		Self::Index(index)
	}
}

impl From<SourceType> for ServerId {
	fn from(source: SourceType) -> Self {
		Self::Source(source)
	}
}
