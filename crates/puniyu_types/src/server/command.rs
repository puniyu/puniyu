use std::sync::OnceLock;
use super::types::ServerCommand;

pub static SERVER_COMMAND_TX: OnceLock<flume::Sender<ServerCommand>> = OnceLock::new();

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
