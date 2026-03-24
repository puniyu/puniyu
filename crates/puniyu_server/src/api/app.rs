use crate::Response;
use actix_web::{get, Responder};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct AppInfo<'a> {
	name: &'a str,
	version: String,
}

#[get("/info")]
pub async fn info() -> impl Responder {
	let version = puniyu_common::app::app_version();
	let app_name = puniyu_common::app::app_name();
	let info = AppInfo {
		name: app_name,
		version: version.to_string(),
	};

	Response::success(info).pretty()
}
