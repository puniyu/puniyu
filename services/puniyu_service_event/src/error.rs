use puniyu_event::EventType;
use smol_str::SmolStr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("event emitter is not running")]
	NotRunning,
	#[error("handler '{handler}' is already listening to '{event_type}'")]
	AlreadyListening { event_type: EventType, handler: SmolStr },
}