use puniyu_version::Version;
use std::path::Path;
use std::sync::OnceLock;
use std::env::current_dir;

#[derive(Debug)]
pub struct AppInfo {
	pub(crate) name: &'static str,
	pub(crate) version: &'static Version,
	pub(crate) working_dir: &'static Path,
}

impl AppInfo {
	pub const fn new(
		name: &'static str,
		version: &'static Version,
		working_dir: &'static Path,
	) -> Self {
		Self { name, version, working_dir }
	}
	pub fn name(&self) -> &'static str {
		self.name
	}
	pub fn version(&self) -> &'static Version {
		self.version
	}
	pub fn working_dir(&self) -> &'static Path {
		self.working_dir
	}
}

static APP_INFO: OnceLock<AppInfo> = OnceLock::new();

pub fn set_app_info(info: AppInfo) {
	APP_INFO.set(info).expect("App info is already set");
}

pub fn app_info() -> &'static AppInfo {
	APP_INFO.get().expect("App info is not set")
}

pub fn app_name() -> &'static str {
	app_info().name()
}
pub fn app_version() -> &'static Version {
	app_info().version()
}

pub fn app_working_dir() -> &'static Path {
	app_info().working_dir()
}
