mod event_log;
use event_log::EventLog;

use async_trait::async_trait;
use puniyu_api::{pkg_description, pkg_name, pkg_version};
use puniyu_context::PluginContext;
use puniyu_error::AnyError;
use puniyu_event::EventType;
use puniyu_handler::Handler;
use puniyu_service::Service;
use puniyu_service_event::EventEmitter;
use semver::Version;
use std::sync::Arc;

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

	fn description(&self) -> Option<&str> {
		Some(pkg_description!())
	}

	fn using(&self) -> Vec<&str> {
		vec![puniyu_service_event::Service {}.name()]
	}

	async fn on_load(&self, ctx: &PluginContext) -> AnyError {
		let emitter = ctx.require::<EventEmitter>()?;
		let handler: Arc<dyn Handler> = Arc::new(EventLog);
		emitter.on(EventType::Message, Arc::clone(&handler))?;
		Ok(())
	}

	async fn on_unload(&self, ctx: &PluginContext) -> AnyError {
		let emitter = ctx.require::<EventEmitter>()?;
		let handler: Arc<dyn Handler> = Arc::new(EventLog);
		emitter.off(EventType::Message, handler);
		Ok(())
	}
}
