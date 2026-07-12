use puniyu_common::source::SourceType;
use salvo::Router;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct RouterHandle {
	inner: Arc<RwLock<Arc<Router>>>,
}

impl RouterHandle {
	pub fn new(router: Router) -> Self {
		Self { inner: Arc::new(RwLock::new(Arc::new(router))) }
	}

	pub fn get(&self) -> Arc<Router> {
		self.inner.read().expect("RouterHandle lock poisoned").clone()
	}

	pub fn set(&self, router: Router) {
		let mut guard = self.inner.write().expect("RouterHandle lock poisoned");
		*guard = Arc::new(router);
	}

	pub fn take(&self) -> Router {
		let mut guard = self.inner.write().expect("RouterHandle lock poisoned");
		Arc::try_unwrap(std::mem::take(&mut *guard)).unwrap_or_else(|_| Router::new())
	}
}

impl PartialEq for RouterHandle {
	fn eq(&self, other: &Self) -> bool {
		Arc::ptr_eq(&self.inner, &other.inner)
	}
}

impl Debug for RouterHandle {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("RouterHandle").finish_non_exhaustive()
	}
}

#[derive(Clone)]
pub struct ServerInfo {
	/// 服务器来源类型
	pub source: SourceType,
	/// 服务器路由句柄
	pub router: RouterHandle,
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