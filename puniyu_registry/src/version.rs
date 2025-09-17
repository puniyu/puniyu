use std::sync::{LazyLock, Mutex};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

static BACKEND_URL: LazyLock<Mutex<String>> =
	LazyLock::new(|| Mutex::new(String::from("http://127.0.0.1:7777")));

pub fn get_backend_url() -> String {
	BACKEND_URL.lock().unwrap().clone()
}
