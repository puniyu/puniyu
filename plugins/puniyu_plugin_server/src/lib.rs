use async_trait::async_trait;
use puniyu_api::{pkg_name, pkg_version};
use puniyu_config::app::AppConfig;
use puniyu_context::PluginContext;
use puniyu_error::AnyError;
use puniyu_server::{Http, Server, ServerOptions};
use semver::Version;
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Default, Clone, Copy)]
pub struct Plugin;

struct Inner {
	server: Server,
}

#[async_trait]
impl puniyu_plugin_core::Plugin for Plugin {
	fn name(&self) -> &str {
		pkg_name!()
	}

	fn version(&self) -> Version {
		pkg_version!()
	}

	fn priority(&self) -> u32 {
		u32::MAX
	}

	async fn on_start(&self, ctx: &PluginContext) -> AnyError {
		let config = AppConfig::get().server();
		let server = Server::new(ServerOptions {
			host: config.host(),
			port: config.port(),
			shutdown_timeout: Duration::from_secs(10),
		});
		let http = server.http();
		server.start().await?;

		if let Err(error) = ctx.provide(http) {
			server.stop().await?;
			return Err(Box::new(error));
		}
		let inner = Arc::new(Inner { server });
		if let Err(error) = ctx.provide(Arc::clone(&inner)) {
			ctx.remove::<Http>();
			inner.server.stop().await?;
			return Err(Box::new(error));
		}
		Ok(())
	}

	async fn on_unload(&self, ctx: &PluginContext) -> AnyError {
		ctx.require::<Arc<Inner>>()?.server.drain().await?;
		Ok(())
	}

	async fn on_stop(&self, ctx: &PluginContext) -> AnyError {
		if let Some(inner) = ctx.remove::<Arc<Inner>>() {
			inner.server.stop().await?;
		}
		ctx.remove::<Http>();
		Ok(())
	}
}
