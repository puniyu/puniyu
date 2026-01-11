mod message;

pub use message::HookMessageType;
use std::str::FromStr;
mod notion;
pub use notion::HookNotionType;
mod request;
pub use request::HookRequestType;

use crate::event::Event;
use crate::event::EventType;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use strum::{Display, IntoStaticStr};
use crate::handler::HandlerResult;

#[derive(
	Debug, Clone, Display, IntoStaticStr, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord,
)]
pub enum HookType {
	Event(HookEventType),
	Status(StatusType),
}

impl FromStr for HookType {
	type Err = std::convert::Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let parts: Vec<&str> = s.split('.').collect();

		match parts.as_slice() {
			["event"] => Ok(HookType::Event(HookEventType::default())),
			["event", sub_type] => {
				let event_type = HookEventType::from_str(sub_type).unwrap_or_default();
				Ok(HookType::Event(event_type))
			}
			["status"] => Ok(HookType::Status(StatusType::default())),
			["status", sub_type] => {
				let status_type = StatusType::from_str(sub_type).unwrap_or_default();
				Ok(HookType::Status(status_type))
			}
			_ => Ok(HookType::Event(HookEventType::default())),
		}
	}
}

impl Default for HookType {
	fn default() -> Self {
		HookType::Event(HookEventType::default())
	}
}

#[derive(
	Debug,
	Clone,
	Default,
	Display,
	IntoStaticStr,
	Deserialize,
	Serialize,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
)]
pub enum StatusType {
	#[default]
	Start,
	Stop,
}

impl FromStr for StatusType {
	type Err = std::convert::Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"start" => Ok(Self::Start),
			"stop" => Ok(Self::Stop),
			_ => Ok(Self::default()),
		}
	}
}

#[derive(
	Debug,
	Clone,
	Default,
	Display,
	IntoStaticStr,
	Deserialize,
	Serialize,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
)]
pub enum HookEventType {
	Message,
	Notion,
	Request,
	#[default]
	All
}

impl From<EventType> for HookEventType {
	fn from(event_type: EventType) -> Self {
		match event_type {
			EventType::Message => HookEventType::Message,
			EventType::Notion => HookEventType::Notion,
			EventType::Request => HookEventType::Request,
			EventType::Unknown => HookEventType::All,
		}
	}
}

impl FromStr for HookEventType {
	type Err = std::convert::Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"message" => Ok(Self::Message),
			"notion" => Ok(Self::Notion),
			"request" => Ok(Self::Request),
			"all" => Ok(Self::All),
			_ => Ok(Self::default()),
		}
	}
}

#[async_trait]
pub trait HookBuilder: Send + Sync + 'static {
	fn name(&self) -> &'static str;

	fn r#type(&self) -> HookType;

	fn rank(&self) -> u32;

	async fn run(
		&self,
		event: Option<&Event>,
	) -> HandlerResult;
}
