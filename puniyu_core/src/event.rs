use strum::{Display, EnumString, IntoStaticStr};
mod cd;
pub mod contact;
mod handler;
pub mod key;
mod matcher;
pub mod message;
pub mod sender;

/// 事件类型枚举
#[derive(Debug, Clone, EnumString, Display, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum EventType {
    Message,
    Notice,
    Request,
    Unknown,
}
