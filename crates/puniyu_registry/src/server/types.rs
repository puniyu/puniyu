use puniyu_types::server::ServerFunction;
use crate::SourceType;


#[derive(Debug, Clone)]
pub struct ServerInfo {
	pub source: SourceType,
	pub builder: ServerFunction,
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
