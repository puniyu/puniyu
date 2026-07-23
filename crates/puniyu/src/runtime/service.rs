use log::debug;
use puniyu_context::{AppContext, ServiceContext};
use puniyu_service::Service;
use std::sync::Arc;

struct ServiceInstance {
	service: Arc<dyn Service>,
	context: ServiceContext,
	started: bool,
}

pub(crate) struct ServiceRuntime {
	app_context: Arc<AppContext>,
	services: Vec<ServiceInstance>,
}

impl ServiceRuntime {
	pub(crate) fn new(app_context: Arc<AppContext>, services: Vec<Arc<dyn Service>>) -> Self {
		let services: Vec<ServiceInstance> = services
			.into_iter()
			.map(|service| {
				let context = ServiceContext::new(Arc::clone(&app_context), service.name());
				ServiceInstance { service, context, started: false }
			})
			.collect();
		Self { app_context, services }
	}

	pub(crate) async fn start(&mut self) {
		for instance in &mut self.services {
			let name = instance.service.name();
			debug!("service '{name}' setting up...");
			if let Err(e) = instance.service.setup(&instance.context).await {
				log::error!("service '{name}' setup failed: {e}");
				continue;
			}
			debug!("service '{name}' ready");
			instance.started = true;
		}
	}

	pub(crate) async fn shutdown(&mut self) {
		for instance in self.services.iter_mut().rev() {
			if instance.started {
				let name = instance.service.name();
				if let Err(e) = instance.service.cleanup(&instance.context).await {
					log::error!("service '{name}' cleanup failed: {e}");
				}
			}
		}
		for instance in self.services.iter().rev() {
			self.app_context.remove_scope(instance.context.scope_id());
		}
	}
}
