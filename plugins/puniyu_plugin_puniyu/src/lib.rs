use async_trait::async_trait;
use convert_case::{Case, Casing};
use figlet_rs::FIGlet;
use puniyu_api::{pkg_name, pkg_version};
use puniyu_context::PluginContext;
use puniyu_error::AnyError;
use semver::Version;
use std::io;
use std::path::PathBuf;
use std::sync::Mutex;

#[derive(Debug)]
struct Inner {
	name: &'static str,
	cwd_dir: &'static PathBuf,
	version: Version,
	git_sha: &'static str,
	repo: &'static str,
}

static CWD_DIR: PathBuf = PathBuf::new();
static INFO: Mutex<Option<Inner>> = Mutex::new(None);

pub struct Plugin;

impl Plugin {
	pub fn with_name(name: &'static str) -> Self {
		Self.with(|info| info.name = name)
	}

	pub fn with_cwd_dir(self, cwd_dir: &'static PathBuf) -> Self {
		self.with(|info| info.cwd_dir = cwd_dir)
	}

	pub fn with_version(self, version: Version) -> Self {
		self.with(|info| info.version = version)
	}

	pub fn with_git_sha(self, git_sha: &'static str) -> Self {
		self.with(|info| info.git_sha = git_sha)
	}

	pub fn with_repo(self, repo: &'static str) -> Self {
		self.with(|info| info.repo = repo)
	}

	fn with(self, f: impl FnOnce(&mut Inner)) -> Self {
		let mut guard = INFO.lock().expect("puniyu info lock poisoned");
		let mut info = guard.take().unwrap_or(Inner {
			name: "",
			cwd_dir: &CWD_DIR,
			version: Version::new(0, 0, 0),
			git_sha: "",
			repo: "",
		});
		f(&mut info);
		guard.replace(info);
		self
	}
}

#[async_trait]
impl puniyu_plugin_core::Plugin for Plugin {
	fn name(&self) -> &str {
		pkg_name!()
	}

	fn priority(&self) -> u32 {
		0
	}

	fn version(&self) -> Version {
		pkg_version!()
	}

	async fn on_start(&self, _ctx: &PluginContext) -> AnyError {
		{
			let guard = INFO.lock().expect("puniyu info lock poisoned");
			let info = guard.as_ref().expect("puniyu info not set");
			puniyu_app::App::init(info.name, info.cwd_dir, info.version.clone());
			start_log(info);
		}
		init_dir().await?;
		Ok(())
	}
}

fn start_log(info: &Inner) {
	let app_name = info.name.to_case(Case::Lower);
	println!("{} starting...", app_name);
	if let Ok(standard_font) = FIGlet::standard()
		&& let Some(art_text) = standard_font.convert(app_name.as_str())
	{
		println!("{}", art_text);
	} else {
		println!("{}", app_name);
	}
	println!("Version: {}", info.version);
	println!("Git SHA: {}", info.git_sha);
	println!("Github: {}", info.repo);
}

async fn init_dir() -> io::Result<()> {
	let dirs = vec![
		puniyu_path::app_dir(),
		puniyu_path::adapter_dir(),
		puniyu_path::data_dir(),
		puniyu_path::config_dir(),
		puniyu_path::resource_dir(),
		puniyu_path::plugin_dir(),
		puniyu_path::log_dir(),
		puniyu_path::temp_dir(),
		puniyu_path::plugin::config_dir(),
		puniyu_path::plugin::data_dir(),
		puniyu_path::plugin::resource_dir(),
		puniyu_path::plugin::temp_dir(),
		puniyu_path::adapter::config_dir(),
		puniyu_path::adapter::data_dir(),
		puniyu_path::adapter::resource_dir(),
		puniyu_path::adapter::temp_dir(),
	];
	for dir in dirs {
		tokio::fs::create_dir_all(dir).await?;
	}
	Ok(())
}
