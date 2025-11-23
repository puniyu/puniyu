mod api;
mod logger;
mod middleware;
mod server;

use std::sync::OnceLock;

pub use server::{run_server, run_server_spawn, stop_server};

pub static LOGO: OnceLock<Vec<u8>> = OnceLock::new();