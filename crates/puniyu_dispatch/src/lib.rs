mod error;
mod store;
#[doc(inline)]
pub use error::Error;

use std::future::Future;
use std::pin::Pin;
use rs_event_emitter::{AsyncEventEmitter, FromArgs, HandlerId, Param, TokioRuntime};
use store::EventEmitterStore;

pub type EventArgs = Vec<Param>;
pub type EventFuture = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

/// 全局事件发射器门面。
pub struct EventEmitter;

impl EventEmitter {
	/// 启动全局事件发射器。
	pub fn run() -> Result<(), Error> {
		EventEmitterStore::run()
	}

	/// 停止全局事件发射器。
	pub fn stop() {
		EventEmitterStore::stop();
	}

	/// 当前全局事件发射器是否已启动。
	pub fn is_running() -> bool {
		EventEmitterStore::is_running()
	}

	/// 监听事件。
	pub async fn on<F, Args>(event: &str, handler: F) -> Result<HandlerId, Error>
	where
		F: Fn(Args) -> EventFuture + Send + Sync + 'static,
		Args: FromArgs + 'static,
	{
		let emitter = EventEmitterStore::emitter()?;
		Ok(emitter.on(event, handler).await)
	}

	/// 卸载事件监听器。
	pub async fn off(event: &str, handler_id: HandlerId) -> Result<(), Error> {
		let emitter = EventEmitterStore::emitter()?;
		emitter.off(event, handler_id).await;
		Ok(())
	}

	/// 触发事件。
	pub async fn emit(event: &str, args: EventArgs) -> Result<(), Error> {
		let emitter = EventEmitterStore::emitter()?;
		emitter.emit(event, args).await;
		Ok(())
	}
}
