use async_trait::async_trait;
use puniyu_logger::error;
use puniyu_registry::HookRegistry;
use puniyu_types::event::Event;
use puniyu_types::handler::{Handler, HandlerResult};
use puniyu_types::hook::HookType;

pub struct HookHandler;

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
			.filter(|x| match x.builder.r#type() {
				HookType::Event(event_type) => event_type == event.event_type().into(),
				_ => false,
			})
			.collect::<Vec<_>>();

		hooks.sort_unstable_by_key(|a| a.builder.rank());
		for hook in hooks {
			if let Err(e) = hook.builder.run(Some(event)).await {
				error!("Hook处理器执行失败: {}", e);
			}
		}
		Ok(())
	}
}
