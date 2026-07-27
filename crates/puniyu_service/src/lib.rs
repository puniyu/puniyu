use std::{any::Any, sync::Arc};

use async_trait::async_trait;

#[async_trait]
pub trait Service: Send + Sync + Any {
	/// Service 名称
	fn name(&self) -> &str;
}

impl<T: Service + ?Sized> Service for Box<T> {
	fn name(&self) -> &str {
		self.as_ref().name()
	}
} 
impl<T: Service + ?Sized> Service for Arc<T> {
	fn name(&self) -> &str {
		self.as_ref().name()
	}
}

impl PartialEq for dyn Service {
	fn eq(&self, other: &Self) -> bool {
		self.name() == other.name()
	}
}

impl Eq for dyn Service {}
