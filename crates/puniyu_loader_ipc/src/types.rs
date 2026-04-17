mod runtime;
mod plugin;
pub(crate) use plugin::*;

use std::{
	borrow::Cow,
	collections::HashMap,
	path::{Path, PathBuf},
	sync::{Arc, RwLock},
};

pub use runtime::ProcessRuntime;

#[derive(Debug, Clone)]
pub enum IpcPluginId<'p> {
	Index(u64),
	Name(Cow<'p, str>),
}

impl From<u64> for IpcPluginId<'_> {
	fn from(value: u64) -> Self {
		Self::Index(value)
	}
}

impl<'p> From<&'p str> for IpcPluginId<'p> {
	fn from(value: &'p str) -> Self {
		Self::Name(Cow::Borrowed(value))
	}
}

impl From<String> for IpcPluginId<'_> {
	fn from(value: String) -> Self {
		Self::Name(Cow::Owned(value))
	}
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ProcessState {
	Starting,
	Running,
	Stopping,
	#[default]
	Stopped,
	Failed,
}

#[derive(Clone)]
pub struct IpcProcess {
	id: u64,
	name: String,
	program: PathBuf,
	args: Vec<String>,
	cwd: Option<PathBuf>,
	env: HashMap<String, String>,
	auto_start: bool,
	runtime: Arc<RwLock<ProcessRuntime>>,
}

impl IpcProcess {
	pub fn new(name: impl Into<String>, program: impl Into<PathBuf>) -> Self {
		Self {
			id: 0,
			name: name.into(),
			program: program.into(),
			args: Vec::new(),
			cwd: None,
			env: HashMap::new(),
			auto_start: false,
			runtime: Arc::new(RwLock::new(ProcessRuntime::default())),
		}
	}

	pub fn from_path<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
		let path = path.as_ref();
		let name = path.file_stem().and_then(|s| s.to_str()).ok_or_else(|| {
			std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid plugin binary path")
		})?;
		Ok(Self::new(name, path.to_path_buf()))
	}

	pub fn with_args(mut self, args: Vec<String>) -> Self {
		self.args = args;
		self
	}

	pub fn with_cwd(mut self, cwd: impl Into<PathBuf>) -> Self {
		self.cwd = Some(cwd.into());
		self
	}

	pub fn with_env(mut self, env: HashMap<String, String>) -> Self {
		self.env = env;
		self
	}

	pub fn with_auto_start(mut self, auto_start: bool) -> Self {
		self.auto_start = auto_start;
		self
	}

	pub fn set_state(&self, state: ProcessState) {
		self.runtime.write().expect("Failed to acquire lock").state = state;
	}

	pub fn set_pid(&self, pid: Option<u32>) {
		self.runtime.write().expect("Failed to acquire lock").pid = pid;
	}

	pub(crate) fn set_id(&mut self, id: u64) {
		self.id = id;
	}
}

impl IpcProcess {
	pub fn id(&self) -> u64 {
		self.id
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn program(&self) -> &Path {
		&self.program
	}

	pub fn args(&self) -> &[String] {
		&self.args
	}

	pub fn cwd(&self) -> Option<&Path> {
		self.cwd.as_deref()
	}

	pub fn env(&self) -> &HashMap<String, String> {
		&self.env
	}

	pub fn auto_start(&self) -> bool {
		self.auto_start
	}

	pub fn runtime(&self) -> Arc<RwLock<ProcessRuntime>> {
		Arc::clone(&self.runtime)
	}

	pub fn state(&self) -> ProcessState {
		self.runtime.read().expect("Failed to acquire lock").state
	}

	pub fn pid(&self) -> Option<u32> {
		self.runtime.read().expect("Failed to acquire lock").pid
	}
}
