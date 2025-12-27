use actix_web::web::{self, ServiceConfig};
use puniyu_plugin::prelude::*;

#[server]
fn server(cfg: &mut ServiceConfig) {
	let api_routes = |cfg: &mut ServiceConfig| {
		cfg.service(crate::api::message);
	};
	cfg.service(web::scope("/api/bot").configure(api_routes));
}
