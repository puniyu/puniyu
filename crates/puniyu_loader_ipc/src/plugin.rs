use puniyu_version::Version;

include!(concat!(env!("OUT_DIR"), "/puniyu.plugin.rs"));

pub(crate) struct IpcPlugin {
	name: String,
}

impl IpcPlugin {
	pub fn new(name: String) -> Self {
		Self { name }
	}
}
impl puniyu_plugin_core::Plugin for IpcPlugin {
	fn name(&self) -> &str {
		&self.name
	}

	fn version(&self) -> Version {
		todo!()
	}

	fn description(&self) -> Option<&'static str> {
		todo!()
	}
}
