use crate::BaseResponse;
use crate::{api, info, middleware};
use actix_web::dev::ServerHandle;
use actix_web::middleware::{NormalizePath, TrailingSlash};
use actix_web::{App, HttpResponse, HttpServer, web};
use puniyu_common::APP_NAME;

use std::net::IpAddr;
use std::sync::{Arc, LazyLock, Mutex};

static SERVER_HANDLE: LazyLock<Arc<Mutex<Option<ServerHandle>>>> =
	LazyLock::new(|| Arc::new(Mutex::new(None)));

async fn logo() -> HttpResponse {
	let logo_data = crate::LOGO.read().ok().and_then(|guard| guard.clone());

	match logo_data {
		Some(data) if !data.is_empty() => HttpResponse::Ok().content_type("image/png").body(data),
		_ => BaseResponse::<()>::not_found("Logo not found").send_json(),
	}
}

/// 运行服务器
///
/// # 参数
///
/// - `host` - 服务器绑定的 IP 地址
/// - `port` - 服务器绑定的端口号
///
/// # 返回
///
/// 返回 `std::io::Result<()>`，如果服务器启动失败则返回错误
pub async fn run_server(host: IpAddr, port: u16) -> std::io::Result<()> {
	info!("服务器在 {} 运行", format!("{}:{}", host, port));

	let server = HttpServer::new(|| {
		let app = App::new()
			.wrap(middleware::AccessLog)
			.wrap(NormalizePath::new(TrailingSlash::Trim))
			.service(
				#[allow(clippy::unwrap_used)]
				web::resource("/").to(|| async { format!("welcome {}", APP_NAME.get().unwrap()) }),
			)
			.service(web::resource("/logo").route(web::get().to(logo)))
			.service(web::resource("/logo.png").route(web::get().to(logo)))
			.configure(|cfg| {
				cfg.service(web::scope("/api/v1").configure(api::register_routes));
			});

		#[cfg(feature = "registry")]
		{
			use crate::registry::ServerRegistry;
			let configs = ServerRegistry::all();
			configs.into_iter().fold(app, |app, cfg| {
				let builder = cfg.builder.clone();
				app.configure(move |sc| builder(sc))
			})
		}

		#[cfg(not(feature = "registry"))]
		app
	})
	.bind((host, port))?;

	let running_server = server.run();
	let handle = running_server.handle();

	if let Ok(mut guard) = SERVER_HANDLE.lock() {
		*guard = Some(handle);
	}

	running_server.await
}

/// 在新的 tokio 任务中运行服务器
///
/// # 参数
///
/// - `host` - 服务器绑定的 IP 地址
/// - `port` - 服务器绑定的端口号
///
/// # 返回
///
/// 返回 `JoinHandle`，可用于等待或取消服务器任务。
///
/// # 示例
///
/// ```rust,ignore
/// use std::net::IpAddr;
///
/// let host = "127.0.0.1".parse::<IpAddr>().unwrap();
/// let handle = run_server_spawn(host, 8080);
///
/// // 等待服务器完成
/// if let Ok(result) = handle.await {
///     result.ok();
/// }
///
/// // 或者取消服务器
/// handle.abort();
/// ```
pub fn run_server_spawn(host: IpAddr, port: u16) -> tokio::task::JoinHandle<std::io::Result<()>> {
	tokio::spawn(run_server(host, port))
}

/// 停止服务器
///
/// 使用优雅关闭方式停止服务器，等待所有正在处理的请求完成后再关闭。
///
/// # 返回
///
/// 成功返回 `Ok(())`，如果服务器未运行或锁获取失败则返回错误。
///
/// # 示例
///
/// ```rust,ignore
/// // 停止服务器
/// stop_server().await?;
/// ```
pub async fn stop_server() -> std::io::Result<()> {
	let handle = SERVER_HANDLE
		.lock()
		.map_err(|e| std::io::Error::other(e.to_string()))?
		.take()
		.ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "服务器未运行"))?;

	info!("正在停止服务器...");
	handle.stop(true).await;
	Ok(())
}

/// 重启服务器
///
/// 优雅关闭当前服务器后，重新加载 Logo 并在新的任务中启动服务器。
///
/// # 参数
///
/// - `host` - 服务器绑定的 IP 地址
/// - `port` - 服务器绑定的端口号
///
/// # 返回
///
/// 成功返回 `Ok(())`，如果停止服务器失败则返回错误。
///
/// # 示例
///
/// ```rust,ignore
/// use std::net::IpAddr;
///
/// let host = "127.0.0.1".parse::<IpAddr>().unwrap();
/// restart_server(host, 8080).await?;
/// ```
pub async fn restart_server(host: IpAddr, port: u16) -> std::io::Result<()> {
	info!("正在重启服务器...");
	stop_server().await?;
	run_server_spawn(host, port);
	Ok(())
}
