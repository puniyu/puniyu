use async_trait::async_trait;
use puniyu_logger::error;
use puniyu_registry::HookRegistry;
use puniyu_types::event::Event;
use puniyu_types::handler::{Handler, HandlerResult, Matcher};
use puniyu_types::hook::HookType;

pub struct HookHandler;

impl Matcher for HookHandler {
	fn matches(&self, _event: &Event) -> bool {
		true
	}
}

#[async_trait]
impl Handler for HookHandler {
	fn name(&self) -> &str {
		"hook"
	}

	fn rank(&self) -> u32 {
		0
	}

	async fn handle(&self, event: &Event) -> HandlerResult {
		let all = HookRegistry::all();
		let mut hooks = all
			.into_iter()
			.filter(|x| match x.r#type() {
				HookType::Event(event_type) => event_type == event.event_type(),
				_ => false,
			})
			.collect::<Vec<_>>();

		hooks.sort_unstable_by_key(|a| a.rank());
		for hook in hooks {
			if let Err(e) = hook.run(Some(event)).await {
				error!("Hook处理器执行失败: {}", e);
			}
		}
		Ok(())
	}
}
