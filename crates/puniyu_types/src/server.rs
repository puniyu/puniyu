use actix_web::web::ServiceConfig;
use std::{
	net::IpAddr,
	sync::{Arc, OnceLock, RwLock},
};

pub type ServerType = Arc<dyn Fn(&mut ServiceConfig) + Send + Sync>;

/// 服务器控制命令
#[derive(Debug, Clone)]
pub enum ServerCommand {
	/// 启动服务器
	Start,
	/// 停止服务器
	Stop,
	/// 重启服务器
	Restart,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
	pub host: IpAddr,
	pub port: u16,
}

pub static SERVER_CONFIG: OnceLock<RwLock<Option<ServerConfig>>> = OnceLock::new();
pub static SERVER_COMMAND_TX: OnceLock<flume::Sender<ServerCommand>> = OnceLock::new();

pub fn save_server_config(host: IpAddr, port: u16) {
	let config = ServerConfig { host, port };
	let store = SERVER_CONFIG.get_or_init(|| RwLock::new(None));
	*store.write().unwrap() = Some(config);
}

pub fn get_server_config() -> Option<ServerConfig> {
	SERVER_CONFIG.get().and_then(|store| store.read().unwrap().clone())
}

pub fn send_server_command(cmd: ServerCommand) -> Result<(), flume::SendError<ServerCommand>> {
	if let Some(tx) = SERVER_COMMAND_TX.get() { tx.send(cmd) } else { Err(flume::SendError(cmd)) }
}

/// 启动服务器
pub fn start_server() -> Result<(), flume::SendError<ServerCommand>> {
	send_server_command(ServerCommand::Start)
}

/// 停止服务器
pub fn stop_server() -> Result<(), flume::SendError<ServerCommand>> {
	send_server_command(ServerCommand::Stop)
}

/// 重启服务器
pub fn restart_server() -> Result<(), flume::SendError<ServerCommand>> {
	send_server_command(ServerCommand::Restart)
}
