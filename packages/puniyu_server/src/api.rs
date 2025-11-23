use std::sync::LazyLock;
use puniyu_common::path::RESOURCE_DIR;

use actix_web::{HttpResponse, web};


async fn logo() -> HttpResponse {
	static LOGO: LazyLock<Option<Vec<u8>>> = LazyLock::new(|| {
		std::fs::read(RESOURCE_DIR.join("logo.png")).ok()
	});
	
	let logo_data = crate::LOGO.get().and_then(|v| if v.is_empty() { None } else { Some(v.clone()) })
		.or_else(|| LOGO.as_ref().cloned());
	
	match logo_data {
		Some(data) => HttpResponse::Ok().content_type("image/png").body(data),
		None => HttpResponse::NotFound().body("Logo not found")
	}
}

pub(crate) fn logo_route(cfg: &mut actix_web::web::ServiceConfig) {
	cfg.service(web::resource("/logo").route(web::get().to(logo)))
	   .service(web::resource("/logo.png").route(web::get().to(logo)));
}
