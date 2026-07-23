use async_trait::async_trait;
use puniyu_api::pkg_version;
use puniyu_context::ServiceContext;
use puniyu_error::AnyError;
use puniyu_task::TaskRegistry;
use semver::Version;

pub const NAME: &str = "puniyu_service_task";

#[derive(Debug, Default, Clone, Copy)]
pub struct Service;

#[async_trait]
impl puniyu_service::Service for Service {
	fn name(&self) -> &str {
		NAME
	}
	fn version(&self) -> Version {
		pkg_version!()
	}
	async fn setup(&self, ctx: &ServiceContext) -> AnyError {
		let registry = TaskRegistry::new();
		registry.start().await?;
		if let Err(error) = ctx.provide(registry.clone()) {
			registry.stop().await?;
			return Err(Box::new(error));
		}
		Ok(())
	}
	async fn cleanup(&self, ctx: &ServiceContext) -> AnyError {
		if let Some(registry) = ctx.remove::<TaskRegistry>() {
			registry.stop().await?;
		}
		Ok(())
	}
}
