use async_trait::async_trait;
use puniyu_api::{pkg_name, pkg_version};
use puniyu_config::{ListConfig, app::AppConfig};
use puniyu_context::PluginContext;
use puniyu_error::AnyError;
use puniyu_event::EventType;
use puniyu_handler::{Handler, HandlerContext};
use puniyu_service::Service;
use puniyu_service_event::EventEmitter;
use semver::Version;
use std::sync::{Arc, OnceLock};

#[derive(Debug, Default)]
pub struct Plugin {
	inner: OnceLock<Arc<Inner>>,
}

impl Plugin {
	pub fn new() -> Self {
		Self::default()
	}
}

#[async_trait]
impl puniyu_plugin_core::Plugin for Plugin {
	fn name(&self) -> &str {
		pkg_name!()
	}

	fn version(&self) -> Version {
		pkg_version!()
	}

	fn using(&self) -> Vec<&str> {
		vec![puniyu_service_event::Service {}.name()]
	}

	async fn on_load(&self, ctx: &PluginContext) -> AnyError {
		let emitter = ctx.require::<EventEmitter>()?;
		let handler: Arc<dyn Handler> = Arc::new(AccessHandler);
		emitter.on(EventType::Message, Arc::clone(&handler))?;
		self.inner.set(Arc::new(Inner { handler: Arc::clone(&handler) })).ok();
		Ok(())
	}

	async fn on_unload(&self, ctx: &PluginContext) -> AnyError {
		let emitter = ctx.require::<EventEmitter>()?;
		if let Some(inner) = self.inner.get() {
			emitter.off(EventType::Message, Arc::clone(&inner.handler));
		}
		Ok(())
	}
}

struct Inner {
	handler: Arc<dyn Handler>,
}

impl std::fmt::Debug for Inner {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Inner").field("handler", &"<handler>").finish()
	}
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

		let app = AppConfig::from_path(puniyu_path::config_dir().join("app").with_extension("toml"));
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
