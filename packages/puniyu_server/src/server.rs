use crate::{api, info, middleware};
use actix_web::middleware::{NormalizePath, TrailingSlash};
use actix_web::{App, HttpServer, web};
use puniyu_common::APP_NAME;
use puniyu_registry::server::ServerRegistry;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::{Arc, OnceLock};
use tokio::sync::Mutex;

static SERVER_HANDLE: OnceLock<Arc<Mutex<Option<actix_web::dev::ServerHandle>>>> = OnceLock::new();

fn get_host_from_env() -> IpAddr {
	std::env::var("HTTP_HOST")
		.ok()
		.and_then(|s| s.parse().ok())
		.unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))
}

fn get_port_from_env() -> u16 {
	std::env::var("HTTP_PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(33720)
}

pub async fn run_server(host: Option<IpAddr>, port: Option<u16>) -> std::io::Result<()> {
	let host = host.unwrap_or_else(get_host_from_env);
	let port = port.unwrap_or_else(get_port_from_env);
	let addr = format!("{}:{}", host, port);

	info!("服务器在 {} 运行", addr);

	let server = HttpServer::new(|| {
		let app = App::new()
			.wrap(middleware::AccessLog)
			.wrap(NormalizePath::new(TrailingSlash::Trim))
			.service(
				web::resource("/").to(|| async { format!("weclome {}", APP_NAME.get().unwrap()) }),
			)
			.configure(api::logo_route);

		ServerRegistry::get_all()
			.into_iter()
			.fold(app, |app, service_cfg| app.configure(|cfg| service_cfg(cfg)))
	})
	.bind(addr)?;

	let running_server = server.run();

	let handle = running_server.handle();
	SERVER_HANDLE.get_or_init(|| Arc::new(Mutex::new(None))).lock().await.replace(handle);

	running_server.await
}

pub async fn stop_server() {
	if let Some(handle_store) = SERVER_HANDLE.get() {
		let guard = handle_store.lock().await;
		if let Some(handle) = &*guard {
			handle.stop(true).await;
		}
	}
}

pub async fn run_server_spawn(host: Option<IpAddr>, port: Option<u16>) {
	tokio::spawn(async move { run_server(host, port).await });
}
