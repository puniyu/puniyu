use async_trait::async_trait;
use puniyu_api::{pkg_name, pkg_version};
use puniyu_config::ConfigRegistry;
use puniyu_context::ServiceContext;
use puniyu_error::AnyError;
use semver::Version;

#[derive(Debug, Default, Clone, Copy)]
pub struct Service;

#[async_trait]
impl puniyu_service::Service for Service {
	fn name(&self) -> &str {
		pkg_name!()
	}

	fn version(&self) -> Version {
		pkg_version!()
	}

	async fn setup(&self, ctx: &ServiceContext) -> AnyError {
		let registry = ConfigRegistry::new();
		ctx.provide(registry)?;
		Ok(())
	}

	async fn cleanup(&self, ctx: &ServiceContext) -> AnyError {
		ctx.remove::<ConfigRegistry>();
		Ok(())
	}
}
