mod app;

pub(crate) fn register_routes(cfg: &mut actix_web::web::ServiceConfig) {
	cfg.service(app::info);
}
