use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::Result;
pub use rmpv::Value;
use rmpv::ext;

pub const PROTOCOL_VERSION: u8 = 1;
pub const DEFAULT_SOCKET_NAME: &str = "puniyu";
pub const MAX_FRAME_SIZE: usize = 8 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MessageKind {
	Request = 0,
	Response = 1,
	Error = 2,
	Cancel = 3,
	Event = 4,
}

impl MessageKind {
	pub fn from_u8(value: u8) -> Option<Self> {
		match value {
			0 => Some(Self::Request),
			1 => Some(Self::Response),
			2 => Some(Self::Error),
			3 => Some(Self::Cancel),
			4 => Some(Self::Event),
			_ => None,
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RequestMessage {
	pub id: u64,
	pub service: String,
	pub params: Value,
}

impl RequestMessage {
	pub fn new<S, P>(id: u64, service: S, params: P) -> Result<Self>
	where
		S: Into<String>,
		P: Serialize,
	{
		Ok(Self {
			id,
			service: service.into(),
			params: ext::to_value(params).map_err(crate::Error::from)?,
		})
	}

	pub fn decode_params<T>(&self) -> Result<T>
	where
		T: DeserializeOwned,
	{
		ext::from_value(self.params.clone()).map_err(crate::Error::from)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseMessage {
	pub id: u64,
	pub result: Value,
}

impl ResponseMessage {
	pub fn new<T>(id: u64, result: T) -> Result<Self>
	where
		T: Serialize,
	{
		Ok(Self {
			id,
			result: ext::to_value(result).map_err(crate::Error::from)?,
		})
	}

	pub fn decode_result<T>(&self) -> Result<T>
	where
		T: DeserializeOwned,
	{
		ext::from_value(self.result.clone()).map_err(crate::Error::from)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ErrorBody {
	pub code: String,
	pub message: String,
}

impl ErrorBody {
	pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
		Self {
			code: code.into(),
			message: message.into(),
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ErrorMessage {
	pub id: u64,
	pub error: ErrorBody,
}

impl ErrorMessage {
	pub fn new(id: u64, code: impl Into<String>, message: impl Into<String>) -> Self {
		Self {
			id,
			error: ErrorBody::new(code, message),
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CancelMessage {
	pub id: u64,
}

impl CancelMessage {
	pub fn new(id: u64) -> Self {
		Self { id }
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EventMessage {
	pub service: String,
	pub params: Value,
}

impl EventMessage {
	pub fn new<S, P>(service: S, params: P) -> Result<Self>
	where
		S: Into<String>,
		P: Serialize,
	{
		Ok(Self {
			service: service.into(),
			params: ext::to_value(params).map_err(crate::Error::from)?,
		})
	}

	pub fn decode_params<T>(&self) -> Result<T>
	where
		T: DeserializeOwned,
	{
		ext::from_value(self.params.clone()).map_err(crate::Error::from)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Message {
	Request(RequestMessage),
	Response(ResponseMessage),
	Error(ErrorMessage),
	Cancel(CancelMessage),
	Event(EventMessage),
}

impl Message {
	pub fn request<S, P>(id: u64, service: S, params: P) -> Result<Self>
	where
		S: Into<String>,
		P: Serialize,
	{
		Ok(Self::Request(RequestMessage::new(id, service, params)?))
	}

	pub fn response<T>(id: u64, result: T) -> Result<Self>
	where
		T: Serialize,
	{
		Ok(Self::Response(ResponseMessage::new(id, result)?))
	}

	pub fn error(id: u64, code: impl Into<String>, message: impl Into<String>) -> Self {
		Self::Error(ErrorMessage::new(id, code, message))
	}

	pub fn cancel(id: u64) -> Self {
		Self::Cancel(CancelMessage::new(id))
	}

	pub fn event<S, P>(service: S, params: P) -> Result<Self>
	where
		S: Into<String>,
		P: Serialize,
	{
		Ok(Self::Event(EventMessage::new(service, params)?))
	}

	pub fn kind(&self) -> MessageKind {
		match self {
			Self::Request(_) => MessageKind::Request,
			Self::Response(_) => MessageKind::Response,
			Self::Error(_) => MessageKind::Error,
			Self::Cancel(_) => MessageKind::Cancel,
			Self::Event(_) => MessageKind::Event,
		}
	}
}
