use puniyu_event::EventType;
use smol_str::SmolStr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("event emitter is not running")]
	NotRunning,
	#[error("listener '{listener}' is already registered for '{event_type}'")]
	AlreadyListening { event_type: EventType, listener: SmolStr },
}
