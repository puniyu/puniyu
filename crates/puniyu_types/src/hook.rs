use crate::event::{Event, EventType};
use async_trait::async_trait;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HookType {
	Event(EventType),
	Status(StatusType),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum StatusType {
	Start,
	Stop,
}

impl Default for HookType {
	fn default() -> Self {
		Self::Event(EventType::Message)
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
	) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}
