mod api;
mod logger;
mod middleware;
mod server;

use serde::{Deserialize, Serialize};
use serde_json::{Value, to_string_pretty};
use actix_web::{HttpResponse, http::StatusCode};
use std::sync::OnceLock;

pub use server::{run_server, run_server_spawn, stop_server};

pub static LOGO: OnceLock<Vec<u8>> = OnceLock::new();

#[derive(Serialize, Deserialize)]
pub struct BaseResponse {
	pub code: u16,
	pub data: Option<Value>,
	pub message: String,
}

impl BaseResponse {
	pub fn send_json(&self) -> HttpResponse {
		let status = StatusCode::from_u16(self.code).unwrap();
		HttpResponse::build(status)
			.content_type("application/json")
			.body(to_string_pretty(self).unwrap())
	}
}
