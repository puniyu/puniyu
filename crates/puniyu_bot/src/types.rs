mod connection;
#[doc(inline)]
pub use connection::ConnProtocol;
mod platform;
#[doc(inline)]
pub use platform::Platform;
mod protocol;
#[doc(inline)]
pub use protocol::Protocol;
mod standard;
#[doc(inline)]
pub use standard::Standard;
mod connection_info;
#[doc(inline)]
pub use connection_info::ConnectionInfo;
mod adapter_info;
#[doc(inline)]
pub use adapter_info::AdapterInfo;

use std::time::Duration;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct SendMsgResult {
	/// 消息 ID
	pub message_id: SmolStr,
	/// 发送时间戳，单位为秒
	pub time: Duration,
}

impl SendMsgResult {
    pub fn new(message_id: impl Into<SmolStr>, time: impl Into<Duration>) -> Self {
        Self {
            message_id: message_id.into(),
            time: time.into(),
        }
    }
}
