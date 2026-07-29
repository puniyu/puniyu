use bytes::{Bytes, BytesMut};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use crate::{ServiceName, error::Error};

const PROTOCOL_VERSION: u8 = const_str::parse!(env!("CARGO_PKG_VERSION_MAJOR"), u8);

#[derive(
	Debug, Clone, Copy, PartialEq, Eq, serde_repr::Serialize_repr, serde_repr::Deserialize_repr,
)]
#[repr(u8)]
pub enum FrameType {
	Message = 0,
	Event = 1,
}

/// 请求帧
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
	pub version: u8,
	#[serde(rename = "type")]
	pub frame_type: FrameType,
	pub service: ServiceName,
	pub payload: Bytes,
}

impl Request {
	pub fn build(service: &ServiceName, payload: &impl Serialize) -> Result<Self, Error> {
		let bytes = Bytes::from(rmp_serde::encode::to_vec(payload)?);
		Ok(Self {
			version: PROTOCOL_VERSION,
			frame_type: FrameType::Message,
			service: service.clone(),
			payload: bytes,
		})
	}
	pub async fn from_reader<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self, Error> {
		let mut len_buf = [0u8; 4];
		reader.read_exact(&mut len_buf).await.map_err(Error::Io)?;
		let len = u32::from_be_bytes(len_buf) as usize;
		let mut payload = BytesMut::with_capacity(len);
		payload.resize(len, 0);
		reader.read_exact(&mut payload).await.map_err(Error::Io)?;
		let req: Self = rmp_serde::from_slice(&payload).map_err(Error::Decode)?;
		Ok(req)
	}
}

/// 响应帧
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
	pub version: u8,
	#[serde(rename = "type")]
	pub frame_type: FrameType,
	pub success: bool,
	pub payload: Bytes,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub error: Option<String>,
}

impl Response {
	/// 构建成功响应
	pub fn success(payload: Bytes) -> Self {
		Self {
			version: PROTOCOL_VERSION,
			frame_type: FrameType::Message,
			success: true,
			payload,
			error: None,
		}
	}
	/// 构建错误响应
	pub fn error(err: impl Into<String>) -> Self {
		Self {
			version: PROTOCOL_VERSION,
			frame_type: FrameType::Message,
			success: false,
			payload: Bytes::new(),
			error: Some(err.into()),
		}
	}
	/// 构建事件推送
	pub fn event(payload: Bytes) -> Self {
		Self {
			version: PROTOCOL_VERSION,
			frame_type: FrameType::Event,
			success: true,
			payload,
			error: None,
		}
	}


	pub async fn to_writer<W: AsyncWrite + Unpin>(&self, writer: &mut W) -> Result<(), Error> {
		let len = (self.payload.len() as u32).to_be_bytes();
		writer.write_all(&len).await.map_err(Error::Io)?;
		writer.write_all(&self.payload).await.map_err(Error::Io)?;
		writer.flush().await.map_err(Error::Io)?;
		Ok(())
	}
}
