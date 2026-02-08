use crate::SourceType;
use puniyu_types::server::ServerFunction;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Clone)]
pub struct ServerInfo {
	pub source: SourceType,
	pub builder: ServerFunction,
}

impl PartialEq for ServerInfo {
	fn eq(&self, other: &Self) -> bool {
		&self.source == &other.source && Arc::as_ptr(&self.builder) == Arc::as_ptr(&other.builder)
	}
}

impl Debug for ServerInfo {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ServerInfo").field("source", &self.source).finish()
	}
}

pub(crate) enum ServerId {
	Index(u64),
	Source(SourceType),
}

impl From<u64> for ServerId {
	fn from(index: u64) -> Self {
		Self::Index(index)
	}
}

impl From<SourceType> for ServerId {
	fn from(source: SourceType) -> Self {
		Self::Source(source)
	}
}
