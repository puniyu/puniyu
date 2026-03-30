use async_trait::async_trait;
use puniyu_context::EventContext;
use puniyu_event::Event;
use puniyu_event::EventBase;
use puniyu_handler::Handler;
use puniyu_hook::HookEventType;
use puniyu_hook::HookRegistry;
use puniyu_hook::HookType;
use puniyu_logger::error;

pub struct HookHandler;

#[async_trait]
impl Handler for HookHandler {
	fn name(&self) -> &'static str {
		"hook"
	}

	fn priority(&self) -> u32 {
		0
	}

	async fn handle(&self, event: &Event) -> puniyu_error::Result {
		let context = EventContext::new(event);
		let event_type = HookEventType::from(*context.event_type());
		let mut hooks = HookRegistry::all();
		hooks.retain(|x| matches!(x.builder.r#type(),
			HookType::Event(t) if *t == HookEventType::All || *t == event_type
		));
		hooks.sort_unstable_by_key(|a| a.builder.priority());
		for hook in hooks {
			if let Err(e) = hook.builder.run(Some(&context)).await {
				error!("Hook处理器执行失败: {}", e);
			}
		}
		Ok(())
	}
}
