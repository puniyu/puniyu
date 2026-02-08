use crate::event::Event;
use std::sync::Arc;
use std::sync::OnceLock;
use tokio::sync::mpsc;

pub type EventSender<'e> = mpsc::Sender<Arc<Event<'e>>>;
pub type EventReceiver<'e> = mpsc::Receiver<Arc<Event<'e>>>;

pub trait EventBus: Send + Sync {
	fn run(&self) -> tokio::task::JoinHandle<()>;

	fn send_event(&self, event: Arc<Event>);

	fn shutdown(&self);
}

pub static EVENT_BUS: OnceLock<Arc<dyn EventBus>> = OnceLock::new();

#[cfg(feature = "bus")]
#[macro_export]
macro_rules! send_event {
	($event:ident) => {{
		use std::sync::Arc;
		let bus = $crate::bus::EVENT_BUS.get().unwrap();
		let event = Arc::from($event);
		bus.send_event(event);
	}};
}
