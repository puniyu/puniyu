mod store;
mod registry;
pub use registry::ServerRegistry;

use actix_web::web::ServiceConfig;
use std::sync::Arc;

pub type ServerType = Arc<dyn Fn(&mut ServiceConfig) + Send + Sync>;
