use crate::event::{EventType, Event};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HookType {
	Event(EventType),
	Status(StatusType)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum StatusType {
	Start,
	Stop
}

impl Default for HookType {
	fn default() -> Self {
		Self::Event(EventType::Message)
	}
}

pub trait HookBuilder: Send + Sync {
	fn name(&self) -> &str;

	fn r#type(&self) -> HookType;

	fn rank(&self) -> u32;

	fn run(&self, event: Option<&Event>) -> Result<(), Box<dyn std::error::Error>>;
}
