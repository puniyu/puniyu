use actix_web::web::ServiceConfig;
use std::sync::Arc;

pub type ServerType = Arc<dyn Fn(&mut ServiceConfig) + Send + Sync>;
