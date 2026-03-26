mod info;
mod logo;

pub(crate) fn register_routes(cfg: &mut actix_web::web::ServiceConfig) {
	use actix_web::web;
	cfg.service(info::info);
	cfg.service(web::resource("/logo").route(web::get().to(logo::logo)));
}
