use bytes::{Buf, BytesMut};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use crate::error::Error;
use crate::frame::{Frame, FrameType};

/// 从 AsyncRead 读取一帧
pub async fn read_frame<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Frame, Error> {
	let len = reader.read_u32().await.map_err(Error::Io)? as usize;

	if len == 0 {
		return Err(Error::EmptyFrame);
	}

	let mut buf = BytesMut::with_capacity(len);
	buf.resize(len, 0);
	reader.read_exact(&mut buf).await.map_err(Error::Io)?;

	let frame_type = buf[0];
	buf.advance(1);

	let body = buf.freeze();

	let frame = match frame_type {
		0 => Frame::Request(rmp_serde::from_slice(&body).map_err(Error::Decode)?),
		1 => Frame::Response(rmp_serde::from_slice(&body).map_err(Error::Decode)?),
		2 => Frame::Notify(rmp_serde::from_slice(&body).map_err(Error::Decode)?),
		3 => Frame::Event(rmp_serde::from_slice(&body).map_err(Error::Decode)?),
		_ => return Err(Error::UnknownFrameType(frame_type)),
	};

	Ok(frame)
}

/// 向 AsyncWrite 写入一帧
pub async fn write_frame<W: AsyncWrite + Unpin>(writer: &mut W, frame: &Frame) -> Result<(), Error> {
	let (frame_type, body) = match frame {
		Frame::Request(req) => (
			FrameType::Request as u8,
			rmp_serde::to_vec(req).map_err(Error::Encode)?,
		),
		Frame::Response(res) => (
			FrameType::Response as u8,
			rmp_serde::to_vec(res).map_err(Error::Encode)?,
		),
		Frame::Notify(notif) => (
			FrameType::Notify as u8,
			rmp_serde::to_vec(notif).map_err(Error::Encode)?,
		),
		Frame::Event(ev) => (
			FrameType::Event as u8,
			rmp_serde::to_vec(ev).map_err(Error::Encode)?,
		),
	};

	let len = 1 + body.len();

	writer.write_u32(len as u32).await.map_err(Error::Io)?;
	writer.write_u8(frame_type).await.map_err(Error::Io)?;
	writer.write_all(&body).await.map_err(Error::Io)?;
	writer.flush().await.map_err(Error::Io)?;

	Ok(())
}
