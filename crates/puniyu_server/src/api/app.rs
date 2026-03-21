use crate::Response;
use actix_web::{get, Responder};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct AppInfo<'a> {
	name: &'a str,
	#[serde(flatten)]
	version: &'a puniyu_version::Version,
}

#[get("/info")]
pub async fn info() -> impl Responder {
	let version = puniyu_common::app::app_version();
	let app_name = puniyu_common::app::app_name();
	let info = AppInfo {
		name: app_name,
		version,
	};

	Response::success(info)

}
