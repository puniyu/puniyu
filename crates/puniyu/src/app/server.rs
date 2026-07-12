use puniyu_server::ServerRegistry;
use salvo::Router;
use std::sync::Arc;

pub fn init_server(
	source: puniyu_server::SourceType,
	router: Router,
) -> Result<(), puniyu_server::Error> {
	ServerRegistry::register(source, router)?;
	Ok(())
}

pub struct ArcHandler(Arc<dyn salvo::Handler>);
impl ArcHandler {
	pub fn new(handler: impl salvo::Handler) -> Self {
		Self(Arc::new(handler))
	}
}
impl From<Arc<dyn salvo::Handler>> for ArcHandler {
	fn from(handler: Arc<dyn salvo::Handler>) -> Self {
		Self(handler)
	}
}

#[async_trait::async_trait]
impl salvo::Handler for ArcHandler {
	async fn handle(
		&self,
		req: &mut salvo::Request,
		depot: &mut salvo::Depot,
		res: &mut salvo::Response,
		ctrl: &mut salvo::FlowCtrl,
	) {
		self.0.handle(req, depot, res, ctrl).await
	}
}
