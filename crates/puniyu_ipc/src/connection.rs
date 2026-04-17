use interprocess::local_socket::tokio::prelude::LocalSocketStream;

use crate::codec::{decode_message, encode_message, recv_frame, send_frame};
use crate::error::Result;
use crate::protocol::Message;

pub struct Connection {
	stream: LocalSocketStream,
}

impl Connection {
	pub(crate) fn new(stream: LocalSocketStream) -> Self {
		Self { stream }
	}

	pub async fn send(&mut self, message: &Message) -> Result<()> {
		let frame = encode_message(message)?;
		send_frame(&mut self.stream, &frame).await?;
		Ok(())
	}

	pub async fn recv(&mut self) -> Result<Message> {
		let frame = recv_frame(&mut self.stream).await?;
		decode_message(&frame)
	}
}
