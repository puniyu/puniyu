use crate::Handler;
use puniyu_event::Event;
use std::borrow::Cow;
use std::ops::Deref;
use std::sync::Arc;

/// Handler 的事件处理上下文。
///
/// 上下文允许读取当前事件，并通过 [`HandleContext::next`] 进入洋葱调用链的
/// 下一层。同一层重复调用 `next` 时，只有第一次调用会执行后续 Handler。
pub struct HandleContext<'c, 'e> {
	event: &'c Event<'e>,
	handlers: &'c [Arc<dyn Handler>],
	next_called: bool,
}

impl<'c, 'e> HandleContext<'c, 'e> {
	/// 从事件和按注册顺序排列的 Handler 快照创建调用链入口。
	#[doc(hidden)]
	pub fn new(event: &'c Event<'e>, handlers: &'c [Arc<dyn Handler>]) -> Self {
		Self { event, handlers, next_called: false }
	}

	/// 调用洋葱链中的下一个 Handler。
	///
	/// 到达链尾或同一层重复调用时静默返回。
	pub async fn next(&mut self) {
		if self.next_called {
			return;
		}
		self.next_called = true;

		let Some((handler, handlers)) = self.handlers.split_first() else {
			return;
		};
		let context = Self::new(self.event, handlers);
		handler.handle(context).await;
	}
}

impl<'c, 'e> Deref for HandleContext<'c, 'e> {
	type Target = Event<'e>;

	fn deref(&self) -> &Self::Target {
		self.event
	}
}

/// 处理器标识符。
pub enum HandlerId<'h> {
	/// 注册索引。
	Index(u64),
	/// 处理器名称。
	Name(Cow<'h, str>),
}

impl From<u64> for HandlerId<'_> {
	fn from(index: u64) -> Self {
		Self::Index(index)
	}
}

impl<'h> From<&'h str> for HandlerId<'h> {
	fn from(name: &'h str) -> Self {
		Self::Name(Cow::Borrowed(name))
	}
}

impl From<String> for HandlerId<'_> {
	fn from(name: String) -> Self {
		Self::Name(Cow::Owned(name))
	}
}
