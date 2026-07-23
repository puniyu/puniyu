mod cooldown;
mod executor;
mod handler;
mod invocation;
mod policy;

pub use puniyu_command::CommandRegistry;
use puniyu_service::Service;

use async_trait::async_trait;
use handler::CommandHandler;
use puniyu_api::{pkg_name, pkg_version};
use puniyu_context::PluginContext;
use puniyu_error::AnyError;
use puniyu_event::EventType;
use puniyu_handler::Handler;
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
		vec![
			puniyu_service_command::Service.name(),
			puniyu_service_event::Service.name(),
		]
	}

	async fn on_load(&self, ctx: &PluginContext) -> AnyError {
		let registry = ctx.require::<CommandRegistry>()?;
		let emitter = ctx.require::<EventEmitter>()?;
		let handler: Arc<dyn Handler> = Arc::new(CommandHandler::new(registry.clone()));
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
