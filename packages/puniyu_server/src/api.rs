use crate::BaseResponse;
use actix_web::{HttpResponse, http::StatusCode, web};
use std::sync::LazyLock;
use puniyu_common::path::RESOURCE_DIR;

async fn logo() -> HttpResponse {
	static LOGO: LazyLock<Option<Vec<u8>>> =
		LazyLock::new(|| std::fs::read(RESOURCE_DIR.join("logo.png")).ok());

	let logo_data = crate::LOGO
		.get()
		.and_then(|v| if v.is_empty() { None } else { Some(v.clone()) })
		.or_else(|| LOGO.as_ref().cloned());

	match logo_data {
		Some(data) => HttpResponse::Ok().content_type("image/png").body(data),
		None =>
			BaseResponse {
				code: StatusCode::NOT_FOUND.as_u16(),
				data: None,
				message: "Logo not found".to_string(),
			}
			.send_json()
		}
}

pub(crate) fn logo_route(cfg: &mut actix_web::web::ServiceConfig) {
	cfg.service(web::resource("/logo").route(web::get().to(logo)))
		.service(web::resource("/logo.png").route(web::get().to(logo)));
}
