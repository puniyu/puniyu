use crate::bot::Bot;
use crate::event::Event;
use std::sync::Arc;
use std::sync::OnceLock;
use tokio::sync::mpsc;

pub type EventSender = mpsc::Sender<(Arc<Bot>, Event)>;
pub type EventReceiver = mpsc::Receiver<(Arc<Bot>, Event)>;

pub trait EventBus: Send + Sync {
	fn run(&self) -> tokio::task::JoinHandle<()>;

	fn send_event(&self, bot: Arc<Bot>, event: Event);

	fn shutdown(&self);
}

pub static EVENT_BUS: OnceLock<Arc<dyn EventBus>> = OnceLock::new();

#[cfg(feature = "bus")]
#[macro_export]
macro_rules! send_event {
	($bot:ident, $event:ident) => {{
		let bus = $crate::bus::EVENT_BUS.get().unwrap();
		bus.send_event($bot, $event);
	}};
}
