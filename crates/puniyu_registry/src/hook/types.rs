use std::cmp::PartialEq;
use crate::SourceType;
use puniyu_types::hook::Hook;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Clone)]
pub struct HookInfo {
	pub source: SourceType,
	pub builder: Arc<dyn Hook>,
}

impl Debug for HookInfo {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_tuple("HookInfo").field(&self.source).finish()
	}
}



impl PartialEq for HookInfo {
	fn eq(&self, other: &Self) -> bool {
		&self.source == &other.source && self.builder.name() == other.builder.name()
	}
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

impl<'h> From<&'h str> for HookId<'h> {
	fn from(name: &'h str) -> Self {
		Self::Name(name)
	}
}


impl From<SourceType> for HookId<'_> {
	fn from(source: SourceType) -> Self {
		Self::Source(source)
	}
}
