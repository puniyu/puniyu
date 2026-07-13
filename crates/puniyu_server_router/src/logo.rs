use salvo::{Depot, Response, handler, hyper::body::Bytes};

#[handler]
pub async fn logo(depot: &mut Depot, res: &mut Response) {
	let Ok(data) = depot.get::<Bytes>("logo") else {
		res.status_code(salvo::http::StatusCode::NOT_FOUND);
		return;
	};
	res.add_header("content-type", salvo::http::mime::IMAGE_PNG.as_ref(), true).ok();
	res.write_body(data.clone()).ok();
}
