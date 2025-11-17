use actix_web::{HttpResponse, get};

const LOGO: &[u8] = include_bytes!("../../../logo.png");

#[get("/logo.png")]
async fn logo_png() -> HttpResponse {
	HttpResponse::Ok().content_type("image/png").body(LOGO)
}

#[get("/logo")]
async fn logo() -> HttpResponse {
	HttpResponse::Ok().content_type("image/png").body(LOGO)
}

pub(crate) fn logo_route(cfg: &mut actix_web::web::ServiceConfig) {
	cfg.service(logo).service(logo_png);
}
