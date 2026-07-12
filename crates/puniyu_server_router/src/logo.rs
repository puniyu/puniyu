use std::sync::OnceLock;

use bytes::Bytes;
use salvo::{Response, handler};

static LOGO: OnceLock<Bytes> = OnceLock::new();

pub fn set_logo(data: Bytes) {
	LOGO.set(data).ok();
}


#[handler]
pub async fn logo(res: &mut Response) {
	let Some(data) = LOGO.get() else {
		res.status_code(salvo::http::StatusCode::NOT_FOUND);
		return;
	};
	res.add_header("content-type", salvo::http::mime::IMAGE_PNG.as_ref(), true).ok();
	res.write_body(data.clone()).ok();
}
