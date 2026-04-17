use std::io;

use serde::{Deserialize, Serialize};

const SERVICE_META: &str = "puniyu.plugin.meta";

#[derive(Debug, Serialize, Deserialize)]
struct PluginMeta {
	name: String,
	version: String,
	description: Option<String>,
	author: Vec<String>,
	abi_version: Option<String>,
}

#[tokio::main]
async fn main() -> io::Result<()> {
	let mut connection = puniyu_ipc::connect().await?;

	loop {
		match connection.recv().await.map_err(protocol_error)? {
			puniyu_ipc::Message::Request(request) if request.service == SERVICE_META => {
				let meta = PluginMeta {
					name: env!("CARGO_PKG_NAME").to_string(),
					version: env!("CARGO_PKG_VERSION").to_string(),
					description: Some("puniyu IPC test plugin".to_string()),
					author: vec!["wuliya".to_string()],
					abi_version: Some("0.8.0".to_string()),
				};
				let response = puniyu_ipc::Message::response(request.id, meta).map_err(protocol_error)?;
				connection.send(&response).await.map_err(protocol_error)?;
			}
			puniyu_ipc::Message::Error(error) => {
				return Err(io::Error::other(format!(
					"{}: {}",
					error.error.code, error.error.message
				)));
			}
			_ => {}
		}
	}
}

fn protocol_error(error: impl std::error::Error + Send + Sync + 'static) -> io::Error {
	io::Error::new(io::ErrorKind::InvalidData, error)
}
