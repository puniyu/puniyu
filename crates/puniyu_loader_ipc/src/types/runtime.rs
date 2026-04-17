use super::ProcessState;


#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProcessRuntime {
	pub state: ProcessState,
	pub pid: Option<u32>,
}

impl ProcessRuntime {
	pub fn new(state: ProcessState, pid: u32) -> Self {
		Self { state, pid: Some(pid) }
	}
}
