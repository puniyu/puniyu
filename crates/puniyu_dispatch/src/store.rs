use crate::{AsyncEventEmitter, Error, TokioRuntime};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, OnceLock};
use tokio::runtime::Handle;

static STORE: OnceLock<EventEmitterInner> = OnceLock::new();

pub(crate) struct EventEmitterInner {
	emitter: AsyncEventEmitter<TokioRuntime>,
	running: AtomicBool,
}

impl EventEmitterInner {
	fn new() -> Self {
		Self {
			emitter: AsyncEventEmitter::new(Arc::new(TokioRuntime::new())),
			running: AtomicBool::new(false),
		}
	}

	fn run(&self) {
		self.running.store(true, Ordering::Release);
	}

	fn stop(&self) {
		self.running.store(false, Ordering::Release);
	}

	fn is_running(&self) -> bool {
		self.running.load(Ordering::Acquire)
	}

	fn emitter(&self) -> &AsyncEventEmitter<TokioRuntime> {
		&self.emitter
	}
}

pub(crate) struct EventEmitterStore;

impl EventEmitterStore {
	pub(crate) fn run() -> Result<(), Error> {
		Self::inner()?.run();
		Ok(())
	}

	pub(crate) fn stop() {
		if let Some(inner) = STORE.get() {
			inner.stop();
		}
	}

	pub(crate) fn is_running() -> bool {
		STORE.get().is_some_and(EventEmitterInner::is_running)
	}

	pub(crate) fn emitter() -> Result<&'static AsyncEventEmitter<TokioRuntime>, Error> {
		let inner = Self::inner()?;
		if inner.is_running() {
			Ok(inner.emitter())
		} else {
			Err(Error::NotRunning)
		}
	}

	fn inner() -> Result<&'static EventEmitterInner, Error> {
		Handle::try_current().map_err(|_| Error::MissingTokioRuntime)?;
		Ok(STORE.get_or_init(EventEmitterInner::new))
	}
}
