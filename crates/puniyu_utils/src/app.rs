use std::sync::OnceLock;

pub static APP_NAME: OnceLock<String> = OnceLock::new();
