use crate::{api, middleware};
use actix_web::dev::ServerHandle;
use actix_web::middleware::{NormalizePath, TrailingSlash};
use actix_web::{App, HttpServer, web};
use puniyu_common::app::app_name;
use puniyu_runtime::ServerRuntime;

use crate::logger::info;
use std::net::IpAddr;
use std::sync::{Arc, LazyLock, Mutex};

static SERVER_HANDLE: LazyLock<Arc<Mutex<Option<ServerHandle>>>> =
	LazyLock::new(|| Arc::new(Mutex::new(None)));

pub fn start_server(host: IpAddr, port: u16) -> std::io::Result<ServerRuntime> {
	info!("Server running on {}", format!("{}:{}", host, port));

	let server = HttpServer::new(|| {
		let app = App::new()
			.wrap(middleware::AccessLog)
			.wrap(NormalizePath::new(TrailingSlash::Trim))
			.service(web::resource("/").to(|| async { format!("welcome {}", app_name()) }))
			.configure(|cfg| {
				cfg.service(web::scope("/api/v1").configure(api::register_routes));
			});

		#[cfg(feature = "registry")]
		let app = {
			use crate::registry::ServerRegistry;
			ServerRegistry::all().into_iter().fold(app, |app, cfg| {
				let builder = cfg.builder.clone();
				app.configure(move |sc| builder.call(sc))
			})
		};

		app
	})
	.bind((host, port))?;
	let running_server = server.run();
	let handle = running_server.handle();

	if let Ok(mut guard) = SERVER_HANDLE.lock() {
		*guard = Some(handle.clone());
	}

	let join_handle = tokio::spawn(running_server);
	Ok(ServerRuntime::new(handle, join_handle))
}

pub async fn run_server(host: IpAddr, port: u16) -> std::io::Result<()> {
	start_server(host, port)?.wait().await
}

pub async fn stop_server() -> std::io::Result<()> {
	let handle =
		SERVER_HANDLE.lock().map_err(|e| std::io::Error::other(e.to_string()))?.take().ok_or_else(
			|| std::io::Error::new(std::io::ErrorKind::NotFound, "Server not running"),
		)?;

	handle.stop(true).await;
	Ok(())
}

pub async fn restart_server(host: IpAddr, port: u16) -> std::io::Result<ServerRuntime> {
	stop_server().await?;
	start_server(host, port)
}
