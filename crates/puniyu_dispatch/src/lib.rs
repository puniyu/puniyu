mod error;
mod store;
#[doc(inline)]
pub use error::Error;

use puniyu_event::Event;
use store::DispatchStore;

/// 全局事件发射器，负责将事件分发到所有已注册的处理器。
pub struct EventEmitter;

impl EventEmitter {
	/// 启动事件发射器。
	///
	/// 必须在 Tokio 异步运行时中调用。启动后才能通过 [`EventEmitter::emit`] 发送事件。
	///
	/// # Errors
	///
	/// 若当前不在 Tokio 运行时中，返回 [`Error::MissingTokioRuntime`]。
	pub fn run() -> Result<(), Error> {
		DispatchStore::run()
	}

	/// 停止事件发射器。
	///
	/// 停止后调用 [`EventEmitter::emit`] 将返回 [`Error::NotRunning`]。
	pub fn stop() {
		DispatchStore::stop()
	}

	/// 返回事件发射器当前是否处于运行状态。
	pub fn is_running() -> bool {
		DispatchStore::is_running()
	}

	/// 通过洋葱调用链分发事件。
	///
	/// 处理器按注册顺序进入调用链。Handler 需要调用 `HandleContext::next`
	/// 才会继续传播事件。
	///
	/// # Errors
	///
	/// 若事件发射器未启动，返回 [`Error::NotRunning`]。
	pub async fn emit(event: Event<'_>) -> Result<(), Error> {
		if !DispatchStore::is_running() {
			return Err(Error::NotRunning);
		}
		let handlers = puniyu_handler::HandlerRegistry::all();
		puniyu_handler::HandleContext::new(&event, &handlers).next().await;
		Ok(())
	}
}
