mod cooldown;
mod executor;
mod handler;
mod invocation;
mod policy;
mod registry;

pub use registry::{CommandRegistry, Error};

use async_trait::async_trait;
use handler::CommandMiddleware;
use puniyu_api::{pkg_name, pkg_version};
use puniyu_context::PluginContext;
use puniyu_error::AnyError;
use puniyu_middleware::Middleware;
use puniyu_plugin_event_bus::EventBus;
use semver::Version;
use std::sync::Arc;

pub const NAME: &str = "puniyu_plugin_command";

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

	async fn on_start(&self, ctx: &PluginContext) -> AnyError {
		let registry = CommandRegistry::new();
		ctx.provide(registry)?;
		Ok(())
	}

	async fn on_load(&self, ctx: &PluginContext) -> AnyError {
		let registry = ctx.require::<CommandRegistry>()?;
		let middleware: Arc<dyn Middleware> = Arc::new(CommandMiddleware::new(registry.clone()));
		ctx.require::<EventBus>()?.register(ctx, middleware)?;
		Ok(())
	}

	async fn on_unload(&self, ctx: &PluginContext) -> AnyError {
		ctx.require::<EventBus>()?.unregister(ctx)?;
		Ok(())
	}

	async fn on_stop(&self, ctx: &PluginContext) -> AnyError {
		if let Some(registry) = ctx.remove::<CommandRegistry>() {
			registry.clear()?;
		}
		Ok(())
	}
}
