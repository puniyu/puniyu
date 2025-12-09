use actix_web::{post, Responder};
use serde_json::json;

#[post("/send_msg")]
pub async fn message() -> impl Responder {
	actix_web::web::Json(json!({
        "message": "message"
    }))
}