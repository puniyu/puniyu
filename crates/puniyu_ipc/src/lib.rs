use std::io;

use bytes::Bytes;
pub use interprocess::local_socket::tokio::{Listener, Stream};
use interprocess::local_socket::{GenericNamespaced, ListenerOptions, tokio::prelude::*};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const MAX_FRAME_SIZE: usize = 1024 * 1024 * 1024;

/// 返回 puniyu IPC 使用的本地 socket 名称。
///
/// 该名称基于 `interprocess` 的 namespaced local socket 机制，
/// 适用于当前 crate 的服务端监听和客户端连接。
pub fn socket_name() -> io::Result<interprocess::local_socket::Name<'static>> {
	"puniyu".to_ns_name::<GenericNamespaced>()
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
	pub async fn accept(&self) -> io::Result<Stream> {
		self.listener.accept().await
	}
}

/// 连接到当前 puniyu IPC 服务端。
pub async fn connect() -> io::Result<Stream> {
	Stream::connect(socket_name()?).await
}

/// 接收一帧二进制消息并返回 [`Bytes`]。
///
/// 帧格式为：4 字节大端长度前缀 + payload。
/// 若对端声明的 payload 长度超过 1 GiB，则返回错误。
pub async fn recv_bytes(stream: &mut Stream) -> io::Result<Bytes> {
	let mut len_buf = [0; 4];
	stream.read_exact(&mut len_buf).await?;
	let len = u32::from_be_bytes(len_buf) as usize;
	if len > MAX_FRAME_SIZE {
		return Err(io::Error::new(io::ErrorKind::InvalidData, "payload too large"));
	}

	let mut buf = vec![0; len];
	stream.read_exact(&mut buf).await?;
	Ok(Bytes::from(buf))
}

/// 发送一帧二进制消息。
///
/// 帧格式为：4 字节大端长度前缀 + payload。
/// 若消息体超过 1 GiB，则返回错误。
pub async fn send_bytes(stream: &mut Stream, bytes: &[u8]) -> io::Result<()> {
	if bytes.len() > MAX_FRAME_SIZE {
		return Err(io::Error::new(io::ErrorKind::InvalidInput, "payload too large"));
	}

	let len = bytes.len() as u32;
	stream.write_all(&len.to_be_bytes()).await?;
	stream.write_all(bytes).await
}
