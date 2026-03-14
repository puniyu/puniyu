use crate::BaseResponse;
use actix_web::{get, HttpResponse};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct AppInfo<'a> {
	name: &'a str,
	#[serde(flatten)]
	version: &'a puniyu_version::Version,
}

#[get("/info")]
pub async fn info() -> HttpResponse {
	let version = puniyu_api::app::app_version();
	let app_name = puniyu_api::app::app_name();
	let info = AppInfo {
		name: app_name,
		version,
	};

	BaseResponse::<AppInfo>::ok("success", Some(info)).send_json()

}
