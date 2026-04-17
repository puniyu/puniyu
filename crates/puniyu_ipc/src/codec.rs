use std::io;

use serde::de::DeserializeOwned;

use crate::error::{Error, Result};
use crate::protocol::{MAX_FRAME_SIZE, Message, MessageKind, PROTOCOL_VERSION};

pub fn encode_message(message: &Message) -> Result<Vec<u8>> {
	let payload = match message {
		Message::Request(request) => rmp_serde::to_vec(request)?,
		Message::Response(response) => rmp_serde::to_vec(response)?,
		Message::Error(error) => rmp_serde::to_vec(error)?,
		Message::Cancel(cancel) => rmp_serde::to_vec(cancel)?,
		Message::Event(event) => rmp_serde::to_vec(event)?,
	};

	let frame_len = payload.len() + 2;
	if frame_len > MAX_FRAME_SIZE {
		return Err(Error::FrameTooLarge(frame_len));
	}

	let mut frame = Vec::with_capacity(frame_len + 4);
	frame.extend_from_slice(&(frame_len as u32).to_be_bytes());
	frame.push(PROTOCOL_VERSION);
	frame.push(message.kind() as u8);
	frame.extend_from_slice(&payload);
	Ok(frame)
}

pub fn decode_message(frame: &[u8]) -> Result<Message> {
	if frame.len() < 6 {
		return Err(Error::InvalidFrame("frame too short"));
	}

	let declared = u32::from_be_bytes([frame[0], frame[1], frame[2], frame[3]]) as usize;
	let actual = frame.len() - 4;
	if declared != actual {
		return Err(Error::FrameLengthMismatch { declared, actual });
	}
	if declared > MAX_FRAME_SIZE {
		return Err(Error::FrameTooLarge(declared));
	}

	let version = frame[4];
	if version != PROTOCOL_VERSION {
		return Err(Error::UnsupportedVersion(version));
	}

	let kind = MessageKind::from_u8(frame[5]).ok_or(Error::UnknownKind(frame[5]))?;
	let payload = &frame[6..];

	match kind {
		MessageKind::Request => decode_payload(payload).map(Message::Request),
		MessageKind::Response => decode_payload(payload).map(Message::Response),
		MessageKind::Error => decode_payload(payload).map(Message::Error),
		MessageKind::Cancel => decode_payload(payload).map(Message::Cancel),
		MessageKind::Event => decode_payload(payload).map(Message::Event),
	}
}

fn decode_payload<T>(payload: &[u8]) -> Result<T>
where
	T: DeserializeOwned,
{
	Ok(rmp_serde::from_slice(payload)?)
}

pub(crate) async fn recv_frame(stream: &mut crate::Stream) -> io::Result<Vec<u8>> {
	use tokio::io::AsyncReadExt;

	let mut len_buf = [0; 4];
	stream.read_exact(&mut len_buf).await?;
	let declared = u32::from_be_bytes(len_buf) as usize;
	if declared > MAX_FRAME_SIZE {
		return Err(io::Error::new(io::ErrorKind::InvalidData, "frame too large"));
	}

	let mut frame = vec![0; declared + 4];
	frame[..4].copy_from_slice(&len_buf);
	stream.read_exact(&mut frame[4..]).await?;
	Ok(frame)
}

pub(crate) async fn send_frame(stream: &mut crate::Stream, frame: &[u8]) -> io::Result<()> {
	use tokio::io::AsyncWriteExt;

	stream.write_all(frame).await
}

