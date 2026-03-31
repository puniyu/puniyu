use std::sync::{Arc, LazyLock, RwLock};

use bytes::Bytes;
use puniyu_path::resource_dir;

pub(crate) static LOGO: LazyLock<Arc<RwLock<Option<Bytes>>>> = LazyLock::new(|| {
	let data = std::fs::read(resource_dir().join("logo.png")).ok().map(Bytes::from);
	Arc::new(RwLock::new(data))
});

pub fn set_logo(data: Bytes) {
	if let Ok(mut logo) = LOGO.write() {
		*logo = Some(data);
	}
}
