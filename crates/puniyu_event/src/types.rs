use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

#[derive(
	Debug,
	Clone,
	Hash,
	Copy,
	PartialEq,
	Eq,
	EnumString,
	Display,
	IntoStaticStr,
	Deserialize,
	Serialize,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum EventType {
	Message,
	Notice,
	Request,
}

#[derive(Debug, Clone, Hash, Copy, PartialEq, Eq, Display, Deserialize, Serialize)]
pub enum SubEventType {
	Message(crate::message::SubEventType),
}
