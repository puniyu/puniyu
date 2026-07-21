use crate::Handler;
use puniyu_event::Event;
use std::ops::Deref;
use std::sync::Arc;

/// 处理器的事件处理上下文。
///
/// 上下文允许读取当前事件，并通过 [`HandlerContext::next`] 进入洋葱调用链的
/// 下一层。同一层重复调用 `next` 时，只有第一次调用会执行后续处理器。
pub struct HandlerContext<'c> {
	event: &'c Event,
	handlers: &'c [Arc<dyn Handler>],
	next_called: bool,
}

impl<'c> HandlerContext<'c> {
	/// 从事件和按优先级排序的处理器快照创建调用链入口。
	#[doc(hidden)]
	pub fn new(event: &'c Event, handlers: &'c [Arc<dyn Handler>]) -> Self {
		Self { event, handlers, next_called: false }
	}

	/// 调用洋葱链中的下一个处理器。
	///
	/// 到达链尾或同一层重复调用时静默返回。
	pub async fn next(&mut self) {
		if self.next_called {
			return;
		}
		self.next_called = true;

		let Some((handler, rest)) = self.handlers.split_first() else {
			return;
		};
		let context = Self::new(self.event, rest);
		handler.handle(context).await;
	}
}

impl<'c> Deref for HandlerContext<'c> {
	type Target = Event;

	fn deref(&self) -> &Self::Target {
		self.event
	}
}
