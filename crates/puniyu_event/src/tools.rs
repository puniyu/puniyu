use std::sync::Arc;

use puniyu_logger::error;
use puniyu_logger::owo_colors::OwoColorize;
use puniyu_registry::HandlerRegistry;
use puniyu_types::event::Event;

pub(crate) async fn dispatch_event(event: Arc<Event>) {
	let mut handlers = HandlerRegistry::handlers();
	handlers.sort_unstable_by_key(|a| a.rank());

	for handler in handlers {
		if handler.matches(&event)
			&& let Err(e) = handler.handle(&event).await
		{
			error!("[{}]: 处理器 {} 执行失败: {:?}", "Event".blue(), handler.name(), e);
		}
	}
}
