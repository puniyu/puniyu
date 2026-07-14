use salvo::{Depot, FlowCtrl, Handler, Request, Response, async_trait, hyper::body::Bytes};

pub struct Logo(Bytes);

impl Logo {
	pub const fn new(data: Bytes) -> Self {
		Self(data)
	}
}

#[async_trait]
impl Handler for Logo {
	async fn handle(
		&self,
		req: &mut Request,
		depot: &mut Depot,
		res: &mut Response,
		ctrl: &mut FlowCtrl,
	) {
		depot.insert("logo", self.0.clone());
		ctrl.call_next(req, depot, res).await;
	}
}
