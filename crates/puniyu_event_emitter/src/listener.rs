use std::sync::Arc;

use async_trait::async_trait;
use puniyu_event::Event;

/// 事件监听器接口。
///
/// 实现此 trait 以接收事件通知。监听器按优先级排序（数值越小越先执行），
/// 同优先级内按注册顺序调用。
#[async_trait]
pub trait Listener: Send + Sync {
	/// 监听器名称。
	fn name(&self) -> &str;

	/// 监听器优先级。数值越小越先执行
	fn priority(&self) -> u32 {
		500
	}

	/// 处理事件。
	async fn handle(&self, event: &Event);
}

#[async_trait]
impl<T: Listener + ?Sized> Listener for Box<T> {
	fn name(&self) -> &str {
		self.as_ref().name()
	}
	fn priority(&self) -> u32 {
		self.as_ref().priority()
	}
	async fn handle(&self, event: &Event) {
		self.as_ref().handle(event).await;
	}
}

#[async_trait]
impl<T: Listener + ?Sized> Listener for Arc<T> {
	fn name(&self) -> &str {
		self.as_ref().name()
	}
	fn priority(&self) -> u32 {
		self.as_ref().priority()
	}
	async fn handle(&self, event: &Event) {
		self.as_ref().handle(event).await;
	}
}