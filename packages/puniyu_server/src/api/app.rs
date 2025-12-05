use crate::BaseResponse;
use actix_web::{HttpResponse, get, http::StatusCode};
use puniyu_common::{APP_NAME, VERSION};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct AppInfo {
	name: String,
	version: String,
	channel: String,
}

#[get("/info")]
pub async fn info() -> HttpResponse {
	let veriosn = format!("{}.{}.{}", VERSION.major, VERSION.minor, VERSION.patch);
	let info = AppInfo {
		name: APP_NAME.get().unwrap().to_owned(),
		version: veriosn,
		channel: VERSION.channel.to_string(),
	};

	let response = BaseResponse {
		code: StatusCode::OK.as_u16(),
		data: Some(info),
		message: "success".to_string(),
	};

	response.send_json()
}
