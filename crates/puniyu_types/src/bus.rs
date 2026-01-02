use crate::event::Event;
use std::sync::Arc;
use std::sync::OnceLock;
use tokio::sync::mpsc;

pub type EventSender = mpsc::Sender<Arc<Event>>;
pub type EventReceiver = mpsc::Receiver<Arc<Event>>;


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
		bus.send_event(Arc::from($event));
	}};
}
