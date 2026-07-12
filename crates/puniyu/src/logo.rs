use std::sync::OnceLock;

use bytes::Bytes;

static LOGO: OnceLock<Bytes> = OnceLock::new();

pub fn set_logo(data: Bytes) {
	LOGO.set(data).ok();
}

pub fn get_logo() -> Bytes {
	LOGO.get().cloned().unwrap_or_default()
}
