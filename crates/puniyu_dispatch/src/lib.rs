mod error;
mod store;
#[doc(inline)]
pub use error::Error;

use puniyu_event::Event;
use store::DispatchStore;

pub struct EventEmitter;

impl EventEmitter {
	pub fn run() -> Result<(), Error> {
		DispatchStore::run()
	}

	pub fn stop() {
		DispatchStore::stop();
	}

	pub fn is_running() -> bool {
		DispatchStore::is_running()
	}

	pub async fn emit(event: Event<'_>) -> Result<(), Error> {
		if !DispatchStore::is_running() {
			return Err(Error::NotRunning);
		}
		let mut handlers = puniyu_handler::HandlerRegistry::all();
		handlers.sort_by_key(|h| h.priority());
		for handler in handlers {
			if let Err(e) = handler.handle(&event).await {
				puniyu_logger::error!("[{}] handler error: {}", handler.name(), e);
			}
		}
		Ok(())
	}
}
