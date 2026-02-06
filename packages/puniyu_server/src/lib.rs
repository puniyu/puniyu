mod api;
mod logger;
mod middleware;
mod server;
mod config;

use actix_web::{HttpResponse, http::StatusCode};
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

pub use server::{run_server_spawn, run_server_with_control};

pub static LOGO: OnceLock<Bytes> = OnceLock::new();

#[derive(Serialize, Deserialize)]
pub struct BaseResponse<T> {
	pub code: u16,
	pub data: Option<T>,
	pub message: String,
}

impl Default for BaseResponse<()> {
	fn default() -> Self {
		Self { code: StatusCode::NOT_FOUND.as_u16(), data: None, message: "not fount".to_string() }
	}
}

impl<T> BaseResponse<T> {
	pub fn send_json(&self) -> HttpResponse
	where
		T: Serialize,
	{
		let status = StatusCode::from_u16(self.code).unwrap();
		HttpResponse::build(status)
			.content_type("application/json")
			.body(serde_json::to_string_pretty(self).unwrap())
	}
}
