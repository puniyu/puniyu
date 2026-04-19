use super::ProcessState;


#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProcessRuntime {
	pub(crate) state: ProcessState,
	pub(crate) pid: Option<u32>,
}

impl ProcessRuntime {
	pub fn new(state: ProcessState, pid: u32) -> Self {
		Self { state, pid: Some(pid) }
	}
	pub fn state(&self) -> ProcessState {
		self.state
	}
	pub fn pid(&self) -> Option<u32> {
		self.pid
	}
}
