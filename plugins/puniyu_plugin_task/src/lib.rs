use async_trait::async_trait;
use puniyu_api::pkg_version;
use puniyu_context::PluginContext;
use puniyu_error::AnyError;
use puniyu_task::TaskRegistry;
use semver::Version;

pub const NAME: &str = "puniyu_plugin_task";

#[derive(Debug, Default, Clone, Copy)]
pub struct Plugin;

#[async_trait]
impl puniyu_plugin_core::Plugin for Plugin {
	fn name(&self) -> &str {
		NAME
	}
	fn version(&self) -> Version {
		pkg_version!()
	}
	async fn on_start(&self, ctx: &PluginContext) -> AnyError {
		let registry = TaskRegistry::new();
		registry.start().await?;
		if let Err(error) = ctx.provide(registry.clone()) {
			registry.stop().await?;
			return Err(Box::new(error));
		}
		Ok(())
	}
	async fn on_stop(&self, ctx: &PluginContext) -> AnyError {
		if let Some(registry) = ctx.remove::<TaskRegistry>() {
			registry.stop().await?;
		}
		Ok(())
	}
}
