use rand::RngExt;
use rand::distr::Alphanumeric;
use serde::{Deserialize, Serialize};
use puniyu_adapter::macros::config;

pub(crate) fn make_random_id() -> String {
	rand::rng().sample_iter(&Alphanumeric).take(32).map(char::from).collect()
}

#[derive(Debug, Deserialize, Serialize)]
#[config(name = "app")]
pub struct AppConfig {
	pub(crate) db_url: String,
}

impl Default for AppConfig {
	fn default() -> Self {
		Self {
			db_url: "".to_string(),
		}
	}
}