use actix_web::{Responder, post};
use serde_json::json;

#[post("/send_msg")]
pub async fn message() -> impl Responder {
	actix_web::web::Json(json!({
		"message": "message"
	}))
}
