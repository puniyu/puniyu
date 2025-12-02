use crate::bot::Bot;
use crate::event::Event;
use async_trait::async_trait;

/// 事件处理器
#[async_trait]
pub trait Handler: Send + Sync {
	/// 匹配结果类型
	type MatchResult;

	/// 处理器名称
	fn name(&self) -> &str;
	/// 优先级
	fn rank(&self) -> u8 {
		5
	}
	/// 处理事件
	///
	/// - `bot`: Bot 实例
	/// - `event`: 事件
	/// - `match_result`: 匹配结果（由 Matcher 返回）
	async fn handle(&self, bot: Bot, event: Event, match_result: Option<Self::MatchResult>);
}
