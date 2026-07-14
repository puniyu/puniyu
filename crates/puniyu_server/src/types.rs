use salvo::Router;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SourceType {
	Plugin(u64),
	Adapter(u64),
}

impl SourceType {
	pub fn is_plugin(&self) -> bool {
		matches!(self, SourceType::Plugin(_))
	}
	pub fn is_adapter(&self) -> bool {
		matches!(self, SourceType::Adapter(_))
	}
}

pub struct ServerInfo {
	/// 服务器来源类型
	pub source: SourceType,
	/// 服务器路由
	pub router: Router,
}

impl PartialEq for ServerInfo {
	fn eq(&self, other: &Self) -> bool {
		self.source == other.source
	}
}

impl Debug for ServerInfo {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ServerInfo").field("source", &self.source).finish()
	}
}

pub enum ServerId {
	/// 通过索引标识服务器
	Index(u64),
	/// 通过来源类型标识服务器
	Source(SourceType),
}

impl From<u64> for ServerId {
	#[inline]
	fn from(index: u64) -> Self {
		Self::Index(index)
	}
}

impl From<SourceType> for ServerId {
	#[inline]
	fn from(source: SourceType) -> Self {
		Self::Source(source)
	}
}
