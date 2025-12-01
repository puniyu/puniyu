use crate::{api, info, middleware};
use actix_web::middleware::{NormalizePath, TrailingSlash};
use actix_web::{App, HttpServer, web};
use puniyu_common::APP_NAME;
use puniyu_registry::server::ServerRegistry;
use std::net::{IpAddr, Ipv4Addr};
use puniyu_types::server::{ServerCommand, SERVER_COMMAND_TX, get_server_config, save_server_config};

fn get_host_from_env() -> IpAddr {
	std::env::var("HTTP_HOST")
		.ok()
		.and_then(|s| s.parse().ok())
		.unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))
}

fn get_port_from_env() -> u16 {
	std::env::var("HTTP_PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(33720)
}

pub async fn run_server_with_control(host: Option<IpAddr>, port: Option<u16>) -> std::io::Result<()> {
	use tokio::sync::mpsc;
	let (tx, mut rx) = mpsc::channel::<ServerCommand>(16);
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
					web::resource("/").to(|| async { format!("welcome {}", APP_NAME.get().unwrap()) }),
				)
				.configure(api::logo_route);

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
			cmd = rx.recv() => {
				match cmd {
					Some(ServerCommand::Start) => {
						continue;
					}
					Some(ServerCommand::Stop) => {
						info!("收到停止命令，正在关闭服务器...");
						handle.stop(true).await;
						return Ok(());
					}
					Some(ServerCommand::Restart) => {
						info!("收到重启命令，正在重启服务器...");
						handle.stop(true).await;
						save_server_config(get_host_from_env(), get_port_from_env());
						continue;
					}
					None => {
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
