use std::{
	path::{Path, PathBuf},
	sync::{
		Arc, RwLock,
		atomic::{AtomicU64, Ordering},
	},
};
use super::{ProcessRuntime, ProcessState};

static IPC_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Clone)]
pub struct IpcProcess {
	id: u64,
	name: String,
	program: PathBuf,
	runtime: Arc<RwLock<ProcessRuntime>>,
}

impl IpcProcess {
	pub fn new(name: impl Into<String>, program: impl Into<PathBuf>) -> Self {
		Self {
			id: IPC_ID.fetch_add(1, Ordering::SeqCst),
			name: name.into(),
			program: program.into(),
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

	pub fn set_state(&self, state: ProcessState) {
		self.runtime.write().expect("Failed to acquire lock").state = state;
	}

	pub fn set_pid(&self, pid: Option<u32>) {
		self.runtime.write().expect("Failed to acquire lock").pid = pid;
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
