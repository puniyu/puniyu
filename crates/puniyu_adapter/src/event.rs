pub use puniyu_api::event::*;
use puniyu_event::Event;

pub async fn send_event(event: Event<'_>) {
	let _ = puniyu_dispatch::EventEmitter::emit(event).await;
}
