use async_trait::async_trait;
use puniyu_api::{pkg_description, pkg_name, pkg_version};
use puniyu_context::ServiceContext;
use puniyu_error::AnyError;
use semver::Version;

pub use puniyu_command::CommandRegistry;

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

	fn description(&self) -> Option<&str> {
		Some(pkg_description!())
	}

	async fn setup(&self, ctx: &ServiceContext) -> AnyError {
		let registry = CommandRegistry::new();
		ctx.provide(registry)?;
		Ok(())
	}

	async fn cleanup(&self, ctx: &ServiceContext) -> AnyError {
		if let Some(registry) = ctx.remove::<CommandRegistry>() {
			registry.clear();
		}
		Ok(())
	}
}
