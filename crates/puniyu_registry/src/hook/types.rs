use crate::SourceType;
use puniyu_types::hook::Hook;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct HookInfo {
	pub source: SourceType,
	pub builder: Arc<dyn Hook>,
}

pub(crate) enum HookId<'h> {
	Index(u64),
	Name(&'h str),
	Source(SourceType),
}

impl From<u64> for HookId<'_> {
	fn from(index: u64) -> Self {
		Self::Index(index)
	}
}

impl From<&str> for HookId<'_> {
	fn from(name: &str) -> Self {
		Self::Name(name)
	}
}

impl From<String> for HookId<'_> {
	fn from(name: String) -> Self {
		Self::Name(name.as_str())
	}
}

impl From<SourceType> for HookId<'_> {
	fn from(source: SourceType) -> Self {
		Self::Source(source)
	}
}