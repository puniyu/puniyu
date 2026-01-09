use crate::BaseResponse;
use crate::{api, info, middleware};
use actix_web::middleware::{NormalizePath, TrailingSlash};
use actix_web::{App, HttpResponse, HttpServer, http::StatusCode, web};
use puniyu_common::APP_NAME;
use puniyu_common::path::RESOURCE_DIR;
use puniyu_registry::server::ServerRegistry;
use puniyu_types::server::{
	SERVER_COMMAND_TX, ServerCommand, get_server_config, save_server_config,
};
use std::net::IpAddr;
use std::sync::LazyLock;
use bytes::Bytes;

fn get_host_from_env() -> IpAddr {
	std::env::var("HTTP_HOST").ok().and_then(|s| s.parse().ok()).unwrap()
}

fn get_port_from_env() -> u16 {
	std::env::var("HTTP_PORT").ok().and_then(|s| s.parse().ok()).unwrap()
}

async fn logo() -> HttpResponse {
	static LOGO: LazyLock<Option<Bytes>> =
		LazyLock::new(|| std::fs::read(RESOURCE_DIR.join("logo.png")).ok().map(Bytes::from));

	let logo_data = crate::LOGO
		.get()
		.and_then(|v| if v.is_empty() { None } else { Some(v.clone()) })
		.or_else(|| LOGO.as_ref().cloned());

	match logo_data {
		Some(data) => HttpResponse::Ok().content_type("image/png").body(data),
		None => BaseResponse::<()> {
			code: StatusCode::NOT_FOUND.as_u16(),
			data: None,
			message: "Logo not found".to_string(),
		}
		.send_json(),
	}
}

pub async fn run_server_with_control(
	host: Option<IpAddr>,
	port: Option<u16>,
) -> std::io::Result<()> {
	let (tx, rx) = flume::bounded::<ServerCommand>(16);
	let _ = SERVER_COMMAND_TX.set(tx);
	let init_host = host.unwrap_or_else(get_host_from_env);
	let init_port = port.unwrap_or_else(get_port_from_env);
	save_server_config(init_host, init_port);

	loop {
		let config = get_server_config();
		let host = config.as_ref().map(|c| c.host).unwrap_or_else(get_host_from_env);
		let port = config.as_ref().map(|c| c.port).unwrap_or_else(get_port_from_env);
		let addr = format!("{}:{}", host, port);

		info!("服务器在 {} 运行", addr);

		let server = HttpServer::new(|| {
			let app = App::new()
				.wrap(middleware::AccessLog)
				.wrap(NormalizePath::new(TrailingSlash::Trim))
				.service(
					web::resource("/")
						.to(|| async { format!("welcome {}", APP_NAME.get().unwrap()) }),
				)
				.service(web::resource("/logo").route(web::get().to(logo)))
				.service(web::resource("/logo.png").route(web::get().to(logo)))
				.configure(|cfg| {
					cfg.service(web::scope("/api/v1").configure(api::register_routes));
				});

			ServerRegistry::get_all()
				.into_iter()
				.fold(app, |app, service_cfg| app.configure(|cfg| service_cfg(cfg)))
		})
		.bind(&addr)?;

		let running_server = server.run();
		let handle = running_server.handle();

		tokio::select! {
			result = running_server => {
				return result;
			}
			cmd = rx.recv_async() => {
				match cmd {
					Ok(ServerCommand::Start) => {
						continue;
					}
					Ok(ServerCommand::Stop) => {
						info!("收到停止命令，正在关闭服务器...");
						handle.stop(true).await;
						return Ok(());
					}
					Ok(ServerCommand::Restart) => {
						info!("收到重启命令，正在重启服务器...");
						handle.stop(true).await;
						save_server_config(get_host_from_env(), get_port_from_env());
						continue;
					}
					Err(_) => {
						handle.stop(true).await;
						return Ok(());
					}
				}
			}
		}
	}
}

pub fn run_server_spawn(host: Option<IpAddr>, port: Option<u16>) {
	tokio::spawn(async move { run_server_with_control(host, port).await });
}
