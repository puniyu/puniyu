use async_trait::async_trait;
use log::info;
use puniyu_core::event::EventBase;
use puniyu_element::receive::Elements;
use puniyu_event::message::MessageEvent;
use puniyu_handler::{HandleContext, Handler};
use puniyu_logger::owo_colors::OwoColorize;
use std::time::Instant;

#[derive(Debug, Default, Clone, Copy)]
pub struct LogHandler;

impl LogHandler {
	/// 创建事件日志处理器。
	pub const fn new() -> Self {
		Self
	}
}

#[async_trait]
impl Handler for LogHandler {
	fn name(&self) -> &'static str {
		"log"
	}

	async fn handle(&self, mut ctx: HandleContext<'_, '_>) {
		let Some(message) = ctx.as_message() else {
			ctx.next().await;
			return;
		};

		let event_id = message.event_id().to_owned();
		info!("{}", format_message(message));
		let started_at = Instant::now();
		ctx.next().await;
		let elapsed = started_at.elapsed().as_millis();
		info!("[{}:{}] 处理完成, 耗时{}ms", "Event".yellow(), event_id.green(), elapsed);
	}
}

fn format_message(event: &MessageEvent<'_>) -> String {
	let raw_message = event.elements().iter().map(format_element).collect::<Vec<_>>().join("");

	if let Some(event) = event.as_group() {
		return format!(
			"[{}:{}][{}:{}-{}]: {}",
			"Bot".yellow(),
			event.self_id().green(),
			"GroupMessage".yellow(),
			event.group_id().green(),
			event.user_id().green(),
			raw_message
		);
	}
	if let Some(event) = event.as_group_temp() {
		return format!(
			"[{}:{}][{}:{}-{}]: {}",
			"Bot".yellow(),
			event.self_id().green(),
			"GroupTempMessage".yellow(),
			event.group_id().green(),
			event.user_id().green(),
			raw_message
		);
	}
	if let Some(event) = event.as_guild() {
		return format!(
			"[{}:{}][{}:{}-{}]: {}",
			"Bot".yellow(),
			event.self_id().green(),
			"GuildMessage".yellow(),
			event.guild_id().green(),
			event.user_id().green(),
			raw_message
		);
	}

	format!(
		"[{}:{}][{}:{}]: {}",
		"Bot".yellow(),
		event.self_id().green(),
		"FriendMessage".yellow(),
		event.user_id().green(),
		raw_message
	)
}

fn format_element(element: &Elements) -> String {
	match element {
		Elements::Text(value) => format!("text:{}", value.text),
		Elements::At(value) => format!("at:{}", value.target_id),
		Elements::Reply(value) => format!("reply:{}", value.message_id),
		Elements::Face(value) => format!("face:{}", value.id),
		Elements::Image(value) => {
			format!("image:{}", value.summary.as_deref().unwrap_or(value.file_name.as_str()))
		}
		Elements::File(value) => format!("file:{}", value.file_name),
		Elements::Video(value) => format!("video:{}", value.file_name),
		Elements::Record(value) => format!("record:{}", value.file_name),
		Elements::Json(value) => format!("json:{}", value.data),
		Elements::Xml(value) => format!("xml:{}", value.data),
	}
}
