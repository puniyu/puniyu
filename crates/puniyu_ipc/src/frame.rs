use bytes::Bytes;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use crate::ServiceName;

const PROTOCOL_VERSION: u8 = const_str::parse!(env!("CARGO_PKG_VERSION_MAJOR"), u8);

/// 帧类型
#[derive(
	Debug, Clone, Copy, PartialEq, Eq,
	serde_repr::Serialize_repr, serde_repr::Deserialize_repr,
)]
#[repr(u8)]
pub enum FrameType {
	/// 请求：需要对端响应
	Request = 0,
	/// 响应：对应某个请求
	Response = 1,
	/// 通知：单向，不需要响应
	Notify = 2,
	/// 事件：一对多广播
	Event = 3,
}

/// 请求帧
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
	pub version: u8,
	/// 请求 ID，用于匹配响应
	pub id: u32,
	/// 目标服务
	pub service: ServiceName,
	/// msgpack 编码的参数
	pub payload: Bytes,
}

impl Request {
	pub fn new(id: u32, service: ServiceName, payload: Bytes) -> Self {
		Self {
			version: PROTOCOL_VERSION,
			id,
			service,
			payload,
		}
	}
}

/// 响应帧
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
	pub version: u8,
	/// 对应请求 ID
	pub id: u32,
	/// 是否成功
	pub success: bool,
	/// 响应数据
	pub payload: Bytes,
	/// 错误信息
	#[serde(skip_serializing_if = "Option::is_none")]
	pub error: Option<SmolStr>,
}

impl Response {
	pub fn success(id: u32, payload: Bytes) -> Self {
		Self {
			version: PROTOCOL_VERSION,
			id,
			success: true,
			payload,
			error: None,
		}
	}

	pub fn error(id: u32, err: impl Into<SmolStr>) -> Self {
		Self {
			version: PROTOCOL_VERSION,
			id,
			success: false,
			payload: Bytes::new(),
			error: Some(err.into()),
		}
	}
}

/// 通知帧
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notify {
	pub version: u8,
	/// 目标服务
	pub service: ServiceName,
	/// msgpack 编码的参数
	pub payload: Bytes,
}

impl Notify {
	pub fn new(service: ServiceName, payload: Bytes) -> Self {
		Self {
			version: PROTOCOL_VERSION,
			service,
			payload,
		}
	}
}

/// 事件帧
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
	pub version: u8,
	/// 事件名称
	pub event: SmolStr,
	/// 事件数据
	pub payload: Bytes,
}

impl Event {
	pub fn new(event: SmolStr, payload: Bytes) -> Self {
		Self {
			version: PROTOCOL_VERSION,
			event,
			payload,
		}
	}
}

/// 统一帧枚举
#[derive(Debug, Clone)]
pub enum Frame {
	Request(Request),
	Response(Response),
	Notify(Notify),
	Event(Event),
}
