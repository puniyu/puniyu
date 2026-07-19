use crate::http::HttpInner;
use salvo::async_trait;
use salvo::http::StatusCode;
use salvo::{Depot, FlowCtrl, Handler, Request, Response};
use std::sync::Arc;

pub(crate) struct HttpDispatcher {
	inner: Arc<HttpInner>,
}

impl HttpDispatcher {
	pub(crate) fn new(inner: Arc<HttpInner>) -> Self {
		Self { inner }
	}
}

#[async_trait]
impl Handler for HttpDispatcher {
	async fn handle(
		&self,
		req: &mut Request,
		_depot: &mut Depot,
		res: &mut Response,
		ctrl: &mut FlowCtrl,
	) {
		if !self.inner.is_running() {
			res.status_code = Some(StatusCode::SERVICE_UNAVAILABLE);
			ctrl.skip_rest();
			return;
		}

		let service = self.inner.service.load_full();
		let handler = service.hyper_handler(
			req.local_addr().clone(),
			req.remote_addr().clone(),
			req.scheme().clone(),
			None,
			ctrl.conn().clone(),
			None,
		);
		*res = handler.handle(std::mem::take(req)).await;
		ctrl.skip_rest();
	}
}
