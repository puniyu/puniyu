use crate::{api, middleware};
use actix_web::dev::ServerHandle;
use actix_web::middleware::{NormalizePath, TrailingSlash};
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use puniyu_common::app::app_name;

use crate::logger::info;
use bytes::Bytes;
use puniyu_path::resource_dir;
use std::net::IpAddr;
use std::sync::{Arc, LazyLock, Mutex, RwLock};

static SERVER_HANDLE: LazyLock<Arc<Mutex<Option<ServerHandle>>>> =
	LazyLock::new(|| Arc::new(Mutex::new(None)));


static LOGO: LazyLock<Arc<RwLock<Option<Bytes>>>> = LazyLock::new(|| {
	let data = std::fs::read(resource_dir().join("logo.png")).ok().map(Bytes::from);
	Arc::new(RwLock::new(data))
});

pub fn set_logo(data: Bytes) {
	if let Ok(mut logo) = LOGO.write() {
		*logo = Some(data);
	}
}

async fn logo() -> impl Responder {
	match LOGO.read().ok().and_then(|g| g.clone()) {
		Some(data) => HttpResponse::Ok().content_type(mime::IMAGE_PNG).body(data),
		None => HttpResponse::NotFound().finish(),
	}
}

pub async fn run_server(host: IpAddr, port: u16) -> std::io::Result<()> {
	info!("服务器在 {} 运行", format!("{}:{}", host, port));

	let server = HttpServer::new(|| {
		let app = App::new()
			.wrap(middleware::AccessLog)
			.wrap(NormalizePath::new(TrailingSlash::Trim))
			.service(web::resource("/").to(|| async { format!("welcome {}", app_name()) }))
			.service(web::resource("/logo").route(web::get().to(logo)))
			.configure(|cfg| {
				cfg.service(
					web::scope("/api/v1")
						.configure(api::register_routes)
						.wrap(middleware::PrettyJson),
				);
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
		*guard = Some(handle);
	}

	running_server.await
}

pub fn run_server_spawn(host: IpAddr, port: u16) -> tokio::task::JoinHandle<std::io::Result<()>> {
	tokio::spawn(run_server(host, port))
}

pub async fn stop_server() -> std::io::Result<()> {
	let handle =
		SERVER_HANDLE.lock().map_err(|e| std::io::Error::other(e.to_string()))?.take().ok_or_else(
			|| std::io::Error::new(std::io::ErrorKind::NotFound, "Server not running"),
		)?;

	handle.stop(true).await;
	Ok(())
}

pub async fn restart_server(host: IpAddr, port: u16) -> std::io::Result<()> {
	stop_server().await?;
	run_server_spawn(host, port);
	Ok(())
}
