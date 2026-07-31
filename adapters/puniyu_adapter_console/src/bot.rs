use std::{fs, path::PathBuf};

use puniyu_api::{
	bot::{AdapterInfo, ConnectionInfo, ConnProtocol, Platform, Protocol, Standard, Status},
	element::File,
	error::AnyError,
	pkg_name, pkg_version,
};

pub const BOT_ID: &str = "console";

pub struct Bot {
	assets_dir: PathBuf,
	adapter_info: AdapterInfo,
	connection_info: ConnectionInfo,
}

impl Bot {
	pub fn new(assets_dir: PathBuf) -> Self {
		Self {
			assets_dir,
			adapter_info: AdapterInfo {
				name: pkg_name!().into(),
				version: pkg_version!(),
				platform: Platform::Other,
				protocol: Protocol::Console,
				standard: Standard::Other,
			},
			connection_info: ConnectionInfo::builder()
				.status(Status::Online)
				.conn_protocol(ConnProtocol::Other)
				.address("stdio".into())
				.build(),
		}
	}
}

#[async_trait::async_trait]
impl puniyu_api::bot::Bot for Bot {
	fn self_id(&self) -> &str {
		BOT_ID
	}

	fn name(&self) -> &str {
		BOT_ID
	}

	fn avatar(&self) -> File {
		let bytes = fs::read(self.assets_dir.join("logo.png")).unwrap_or_default();
		File::Bytes(bytes.into())
	}

	fn adapter_info(&self) -> &AdapterInfo {
		&self.adapter_info
	}

	fn connection_info(&self) -> &ConnectionInfo {
		&self.connection_info
	}

	async fn call_api(
		&self,
		_action: &str,
		_params: serde_json::Value,
	) -> AnyError<serde_json::Value> {
		Err("console adapter does not support platform API calls".into())
	}
}
