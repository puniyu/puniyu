use crate::logo::LOGO;
use actix_web::{HttpResponse, Responder};

pub(super) async fn logo() -> impl Responder {
	match LOGO.read().ok().and_then(|g| g.clone()) {
		Some(data) => HttpResponse::Ok().content_type(mime::IMAGE_PNG).body(data),
		None => HttpResponse::NotFound().finish(),
	}
}
