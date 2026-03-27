mod registry;
#[doc(inline)]
pub use registry::AdapterRegistry;
mod types;
#[doc(inline)]
pub use types::*;

use puniyu_adapter_api::AdapterApi;
use puniyu_adapter_types::AdapterInfo;
use puniyu_config::Config;
use puniyu_hook::Hook;
use std::sync::Arc;

#[async_trait::async_trait]
pub trait Adapter: Send + Sync + 'static {
	/// 获取适配器信息
	///
	/// 返回适配器的元数据，包括名称、版本、平台等信息。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let info = adapter.info();
	/// println!("适配器名称: {}", info.name);
	/// println!("平台: {}", info.platform);
	/// ```
	fn info(&self) -> AdapterInfo;

	/// 获取适配器 API
	///
	/// 返回适配器的 API 接口，用于调用各种功能。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// let api = adapter.api();
	///
	/// // 发送消息
	/// api.message().send_msg(&contact, message).await?;
	///
	/// // 获取群列表
	/// let groups = api.group().get_group_list().await?;
	/// ```
	fn api(&self) -> AdapterApi;

	/// 获取配置文件列表
	///
	/// 返回适配器使用的配置文件列表。默认返回空列表。
	///
	/// # 返回值
	///
	/// 返回实现了 `Config` trait 的配置对象列表
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_config::types::Config;
	/// use std::sync::Arc;
	///
	/// impl Adapter for MyAdapter {
	///     fn config(&self) -> Vec<Arc<dyn Config>> {
	///         vec![Arc::new(MyConfig::default())]
	///     }
	/// }
	/// ```
	fn config(&self) -> Vec<Arc<dyn Config>> {
		Vec::new()
	}

	/// 获取钩子列表
	///
	/// 返回适配器注册的钩子列表。默认返回空列表。
	///
	/// # 返回值
	///
	/// 返回实现了 [`Hook`] trait 的钩子对象列表
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_hook::Hook;
	/// use std::sync::Arc;
	///
	/// impl Adapter for MyAdapter {
	///     fn hooks(&self) -> Vec<Arc<dyn Hook>> {
	///         vec![Arc::new(MyHook::default())]
	///     }
	/// }
	/// ```
	fn hooks(&self) -> Vec<Arc<dyn Hook>> {
		Vec::new()
	}

	/// 获取服务器实例
	///
	/// 返回适配器的服务器实例，用于路由管理。默认返回 None。
	///
	/// # 返回值
	///
	/// 返回 `ServerFunction` 用于注册自定义 HTTP 路由
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// use puniyu_server_types::ServerFunction;
	/// use actix_web::web::{ServiceConfig, resource};
	/// use actix_web::HttpResponse;
	/// use std::sync::Arc;
	///
	/// impl Adapter for MyAdapter {
	///     fn server(&self) -> Option<ServerFunction> {
	///         Some(Arc::new(|cfg: &mut ServiceConfig| {
	///             cfg.service(
	///                 resource("/adapter/status").to(|| async {
	///                     HttpResponse::Ok().body("Running")
	///                 })
	///             );
	///         }))
	///     }
	/// }
	/// ```
	fn server(&self) -> Option<puniyu_server::ServerFunction> {
		None
	}

	/// 初始化适配器
	///
	/// 在适配器启动时调用，用于执行初始化逻辑。
	/// 默认实现仅记录日志。
	///
	/// # 错误
	///
	/// 如果初始化失败，返回错误。
	///
	/// # 示例
	///
	/// ```rust,ignore
	/// #[async_trait::async_trait]
	/// impl Adapter for MyAdapter {
	///     async fn init(&self) -> puniyu_error::Result {
	///         // 连接到平台
	///         self.connect().await?;
	///
	///         // 加载配置
	///         self.load_config().await?;
	///
	///         puniyu_logger::info!("适配器初始化完成");
	///         Ok(())
	///     }
	/// }
	/// ```
	async fn init(&self) -> puniyu_error::Result {
		puniyu_logger::info!("Adapter: 初始化完成");
		Ok(())
	}
}

impl PartialEq for dyn Adapter {
	fn eq(&self, other: &Self) -> bool {
		self.info() == other.info() && self.api() == other.api()
	}
}
