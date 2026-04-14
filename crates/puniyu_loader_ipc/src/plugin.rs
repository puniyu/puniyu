include!(concat!(env!("OUT_DIR"), "/puniyu.plugin.rs"));

pub(crate) struct IpcPlugin {
	name: String,
}

impl IpcPlugin {
	pub fn new(name: String) -> Self {
		Self { name }
	}
}
