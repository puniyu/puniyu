mod codec;
mod connection;
mod error;
mod protocol;

use std::io;

pub use connection::Connection;
pub use error::{Error, Result};
use interprocess::local_socket::tokio::prelude::*;
pub use interprocess::local_socket::tokio::{Listener, Stream};
use interprocess::local_socket::{GenericNamespaced, ListenerOptions};
pub use protocol::{
	CancelMessage, DEFAULT_SOCKET_NAME, ErrorBody, ErrorMessage, EventMessage, MAX_FRAME_SIZE,
	Message, MessageKind, PROTOCOL_VERSION, RequestMessage, ResponseMessage, Value,
};

/// 返回 puniyu IPC 使用的本地 socket 名称。
pub fn socket_name() -> io::Result<interprocess::local_socket::Name<'static>> {
	DEFAULT_SOCKET_NAME.to_ns_name::<GenericNamespaced>()
}

/// puniyu IPC 服务端。
pub struct Server {
	listener: Listener,
}

impl Server {
	/// 绑定到 puniyu IPC 本地 socket 并创建服务端。
	pub async fn bind() -> io::Result<Self> {
		let listener = ListenerOptions::new().name(socket_name()?).create_tokio()?;
		Ok(Self { listener })
	}

	/// 接受一个新的 IPC 连接。
	pub async fn accept(&self) -> io::Result<Connection> {
		let stream = self.listener.accept().await?;
		Ok(Connection::new(stream))
	}
}

/// 连接到当前 puniyu IPC 服务端。
pub async fn connect() -> io::Result<Connection> {
	let stream = Stream::connect(socket_name()?).await?;
	Ok(Connection::new(stream))
}
