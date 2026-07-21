use async_trait::async_trait;
use puniyu_api::{pkg_name, pkg_version};
use puniyu_config::{ListConfig, app::AppConfig};
use puniyu_context::PluginContext;
use puniyu_error::AnyError;
use puniyu_event::EventType;
use puniyu_handler::{Handler, HandlerContext};
use puniyu_plugin_event::EventEmitter;
use semver::Version;
use std::sync::Arc;

pub const NAME: &str = "puniyu_plugin_access";

#[derive(Debug, Default, Clone, Copy)]
pub struct Plugin;

#[async_trait]
impl puniyu_plugin_core::Plugin for Plugin {
	fn name(&self) -> &str {
		pkg_name!()
	}

	fn version(&self) -> Version {
		pkg_version!()
	}

	async fn on_start(&self, _ctx: &PluginContext) -> AnyError {
		Ok(())
	}

	async fn on_load(&self, ctx: &PluginContext) -> AnyError {
		let emitter = ctx.require::<EventEmitter>()?;
		let handler: Arc<dyn Handler> = Arc::new(AccessHandler);
		emitter.on(EventType::Message, Arc::clone(&handler))?;
		if let Err(error) = ctx.provide(Arc::new(Inner { handler: Arc::clone(&handler) })) {
			emitter.off(EventType::Message, Arc::clone(&handler));
			return Err(Box::new(error));
		}
		Ok(())
	}

	async fn on_unload(&self, ctx: &PluginContext) -> AnyError {
		let emitter = ctx.require::<EventEmitter>()?;
		if let Some(inner) = ctx.remove::<Arc<Inner>>() {
			emitter.off(EventType::Message, Arc::clone(&inner.handler));
		}
		Ok(())
	}
}

struct Inner {
	handler: Arc<dyn Handler>,
}

#[derive(Debug, Default, Clone, Copy)]
struct AccessHandler;

#[async_trait]
impl Handler for AccessHandler {
	fn name(&self) -> &'static str {
		"access"
	}

	fn priority(&self) -> u32 {
		200
	}

	async fn handle(&self, mut ctx: HandlerContext<'_>) {
		let Some(message) = ctx.as_message() else {
			ctx.next().await;
			return;
		};

		let app = AppConfig::get();
		let allowed = if let Some(group) = message.as_group() {
			is_allowed(&app.group(), group.group_id())
		} else if let Some(group) = message.as_group_temp() {
			is_allowed(&app.group(), group.group_id())
		} else if let Some(guild) = message.as_guild() {
			is_allowed(&app.group(), guild.guild_id())
		} else {
			is_allowed(&app.friend(), message.user_id())
		};

		if allowed {
			ctx.next().await;
		}
	}
}

fn is_allowed(config: &ListConfig, id: &str) -> bool {
	let enabled = config.enable_list();
	if !enabled.is_empty() {
		return enabled.contains(&id);
	}
	!config.disable_list().contains(&id)
}
