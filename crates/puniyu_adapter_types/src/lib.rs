//! # puniyu_adapter_types
//!
//! 统一的 puniyu 适配器类型库，覆盖适配器信息、消息结果与群好友资料场景。
//!
//! ## 特性
//!
//! - 提供 [`AdapterInfo`] 和 `adapter_info!` 宏
//! - 提供 [`MessageType`]、[`SendMsgType`]、[`MessageInfo`]
//! - 提供群聊、成员、好友资料类型
//! - 提供平台、标准、协议与通信方式枚举
//!
//! ## 示例
//!
//! ```rust
//! use puniyu_adapter_types::{adapter_info, AdapterPlatform, AdapterProtocol, MessageType};
//!
//! let info = adapter_info!(
//!     "console",
//!     AdapterPlatform::QQ,
//!     AdapterProtocol::Console
//! );
//!
//! assert_eq!(info.name, "console");
//!
//! let message = MessageType::from("12345");
//! match message {
//!     MessageType::Id(id) => assert_eq!(id, "12345"),
//!     MessageType::Seq(_) => unreachable!(),
//! }
//! ```

mod connection;
#[doc(inline)]
pub use connection::*;
mod platform;
#[doc(inline)]
pub use platform::*;
mod protocol;
#[doc(inline)]
pub use protocol::*;
mod standard;
#[doc(inline)]
pub use standard::*;

use bon::Builder;
use jiff::Timestamp;
use semver::Version;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use std::time::Duration;

#[derive(Debug, Clone, Builder, Deserialize, Serialize)]
#[builder(on(SmolStr, into))]
pub struct AdapterInfo {
	/// 适配器名称
	#[builder(default)]
	pub name: SmolStr,
	/// 适配器作者
	#[builder(default)]
	pub author: Vec<SmolStr>,
	/// 适配器描述
	pub description: Option<SmolStr>,
	/// 适配器版本。
	#[builder(default = AdapterInfo::default_version())]
	pub version: Version,
	/// 适配器平台。
	#[builder(default)]
	pub platform: AdapterPlatform,
	/// 适配器标准。
	#[builder(default)]
	pub standard: AdapterStandard,
	/// 适配器协议实现。
	#[builder(default)]
	pub protocol: AdapterProtocol,
	/// 适配器通信方式。
	#[builder(default)]
	pub conn_protocol: AdapterConnProtocol,
	/// 适配器通信地址
	pub address: Option<String>,
	/// 连接时间。
	#[builder(default = AdapterInfo::default_connect_time())]
	pub connect_time: Timestamp,
	/// 鉴权密钥。
	pub secret: Option<String>,
}

impl PartialEq for AdapterInfo {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
			&& self.author == other.author
			&& self.version == other.version
			&& self.platform == other.platform
			&& self.standard == other.standard
			&& self.protocol == other.protocol
			&& self.conn_protocol == other.conn_protocol
			&& self.address == other.address
			&& self.secret == other.secret
			&& self.connect_time == other.connect_time
	}
}

impl Eq for AdapterInfo {}

impl AdapterInfo {
	const fn default_version() -> Version {
		Version::new(0, 1, 0)
	}

	fn default_connect_time() -> Timestamp {
		Timestamp::now()
	}
}

/// 创建 `AdapterInfo` 的便捷宏。
#[macro_export]
macro_rules! adapter_info {
    ( $( $key:ident : $value:expr ),+ $(,)? ) => {{
        $crate::AdapterInfo::builder()
            $(
                .$key($value)
            )*
            .build()
    }};
	($name:expr, $platform:expr, $protocol:expr) => {{
		adapter_info!(
			name: $name,
			platform: $platform,
			protocol: $protocol,
		)
	}};
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct SendMsgResult {
	/// 消息 ID
	pub message_id: SmolStr,
	/// 发送时间戳，单位为秒
	pub time: Duration,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq, Builder)]
pub struct SendMessageOptions {
	/// 是否回复消息
	#[builder(default)]
	#[serde(default)]
	reply: bool,
}
