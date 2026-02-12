use crate::BaseResponse;
use actix_web::{get, HttpResponse};
use puniyu_common::{APP_NAME, version::VERSION};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct AppInfo {
	name: String,
	version: String,
	channel: String,
}

#[get("/info")]
pub async fn info() -> HttpResponse {
	let version = format!("{}.{}.{}", VERSION.major, VERSION.minor, VERSION.patch);
	let app_name = APP_NAME.get().map_or("Unknown".to_string(), |name| name.to_owned());
	let info = AppInfo {
		name: app_name,
		version,
		channel: VERSION.channel.to_string(),
	};

	BaseResponse::<AppInfo>::ok("success", Some(info)).send_json()

}
