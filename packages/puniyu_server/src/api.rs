use actix_web::{HttpResponse, web};

const LOGO: &[u8] = include_bytes!("../../../logo.png");

async fn logo() -> HttpResponse {
	HttpResponse::Ok().content_type("image/png").body(LOGO)
}

pub(crate) fn logo_route(cfg: &mut actix_web::web::ServiceConfig) {
	cfg.service(web::resource("/logo").route(web::get().to(logo)))
	   .service(web::resource("/logo.png").route(web::get().to(logo)));
}
