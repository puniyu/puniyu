use async_trait::async_trait;

#[derive(Debug, Clone)]
pub enum HandlerResult {
	/// 处理完成，或者是不处理
	Ok,
	/// 继续处理
	Continue,
}

/// 事件处理器
#[async_trait]
pub trait EventHandler: Send + Sync {
	async fn handle(&self) -> HandlerResult;
	fn get_name(&self) -> &str;

	fn rank(&self) -> u8 {
		5
	}
}

pub struct MessageHandler;

#[async_trait]
impl EventHandler for MessageHandler {
	async fn handle(&self) -> HandlerResult {
		HandlerResult::Ok
	}

	fn get_name(&self) -> &str {
		"message"
	}
}
