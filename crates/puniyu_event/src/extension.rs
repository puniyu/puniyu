use crate::EventBase;
use std::any::Any;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ExtensionSubEventType(&'static str);

impl ExtensionSubEventType {
	pub const fn new(kind: &'static str) -> Self {
		Self(kind)
	}

	pub const fn kind(&self) -> &'static str {
		self.0
	}
}

impl From<&'static str> for ExtensionSubEventType {
	fn from(value: &'static str) -> Self {
		Self::new(value)
	}
}

impl std::fmt::Display for ExtensionSubEventType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(self.0)
	}
}

pub trait ExtensionEvent: EventBase + Any + Send + Sync + 'static {}

impl dyn ExtensionEvent {
	pub fn extension<T>(&self) -> Option<&T>
	where
		T: ExtensionEvent + 'static,
	{
		(self as &dyn Any).downcast_ref::<T>()
	}
}

