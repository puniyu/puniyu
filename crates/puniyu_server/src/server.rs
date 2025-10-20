use crate::logger::log_init;
use crate::{info, middleware};
use actix_web::middleware::{NormalizePath, TrailingSlash};
use actix_web::{App, HttpServer};
use std::net::IpAddr;

pub async fn run_server(host: Option<IpAddr>, port: Option<u16>) {
	log_init();
	let host = host.unwrap_or_else(|| {
		std::env::var("HTTP_HOST")
			.ok()
			.and_then(|s| s.parse().ok())
			.unwrap_or(IpAddr::V4([127, 0, 0, 1].into()))
	});

	let port = port.unwrap_or_else(|| {
		std::env::var("HTTP_PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(33720)
	});
	let addr = format!("{}:{}", host, port);
	info!("服务器在 {} 运行", addr);
	HttpServer::new(|| {
		App::new()
			.wrap(middleware::AccessLog)
			.wrap(NormalizePath::new(TrailingSlash::Trim))
			.service(actix_web::web::resource("/").to(|| async { "Hello World!" }))
	})
	.bind(addr)
	.unwrap()
	.run()
	.await
	.unwrap();
}

pub async fn run_server_spawn(host: Option<IpAddr>, port: Option<u16>) {
	tokio::spawn(run_server(host, port));
}
