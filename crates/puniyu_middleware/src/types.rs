use crate::Middleware;
use puniyu_event::Event;
use std::ops::Deref;
use std::sync::Arc;

/// 中间件的事件处理上下文。
///
/// 上下文允许读取当前事件，并通过 [`MiddlewareContext::next`] 进入洋葱调用链的
/// 下一层。同一层重复调用 `next` 时，只有第一次调用会执行后续中间件。
pub struct MiddlewareContext<'c> {
	event: &'c Event,
	middlewares: &'c [Arc<dyn Middleware>],
	next_called: bool,
}

impl<'c> MiddlewareContext<'c> {
	/// 从事件和按优先级排序的中间件快照创建调用链入口。
	#[doc(hidden)]
	pub fn new(event: &'c Event, middlewares: &'c [Arc<dyn Middleware>]) -> Self {
		Self { event, middlewares, next_called: false }
	}

	/// 调用洋葱链中的下一个中间件。
	///
	/// 到达链尾或同一层重复调用时静默返回。
	pub async fn next(&mut self) {
		if self.next_called {
			return;
		}
		self.next_called = true;

		let Some((middleware, rest)) = self.middlewares.split_first() else {
			return;
		};
		let context = Self::new(self.event, rest);
		middleware.handle(context).await;
	}
}

impl<'c> Deref for MiddlewareContext<'c> {
	type Target = Event;

	fn deref(&self) -> &Self::Target {
		self.event
	}
}
