use crate::Error;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::runtime::Handle;

static RUNNING: OnceLock<AtomicBool> = OnceLock::new();

fn running() -> &'static AtomicBool {
	RUNNING.get_or_init(|| AtomicBool::new(false))
}

pub(crate) struct DispatchStore;

impl DispatchStore {
	pub fn run() -> Result<(), Error> {
		Handle::try_current().map_err(|_| Error::MissingTokioRuntime)?;
		running().store(true, Ordering::Release);
		Ok(())
	}

	pub fn stop() {
		running().store(false, Ordering::Release);
	}

	pub fn is_running() -> bool {
		running().load(Ordering::Acquire)
	}
}
