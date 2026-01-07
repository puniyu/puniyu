use crate::event::EventType;

#[derive(Debug, Clone)]
pub enum HookType {
	Event(EventType),
	Start,
	Stop,
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

	fn run(&self) -> Result<(), Box<dyn std::error::Error>>;
}
