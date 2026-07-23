use async_trait::async_trait;
use puniyu_event::message::MessageEvent;
use puniyu_handler::{Handler, HandlerContext};
use puniyu_logger::owo_colors::OwoColorize;
use std::time::Instant;

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct EventLog;

#[async_trait]
impl Handler for EventLog {
	fn name(&self) -> &'static str {
		"event_log"
	}

	fn priority(&self) -> u32 {
		100
	}

	async fn handle(&self, mut ctx: HandlerContext<'_>) {
		let Some(message) = ctx.as_message() else {
			ctx.next().await;
			return;
		};

		let event_id = message.event_id().to_owned();
		log::info!("{}", format_message(message));
		let started_at = Instant::now();
		ctx.next().await;
		let elapsed = started_at.elapsed().as_millis();
		log::info!(
			"[{}:{}] 处理完成, 耗时{}ms",
			"Event".yellow(),
			event_id.green(),
			elapsed
		);
	}
}

fn format_message(event: &MessageEvent) -> String {
	use puniyu_event::EventBase;

	let raw_message =
		event.elements().iter().map(format_element).collect::<Vec<_>>().join("");

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

fn format_element(element: &puniyu_element::receive::Elements) -> String {
	match element {
		puniyu_element::receive::Elements::Text(value) => format!("text:{}", value.text),
		puniyu_element::receive::Elements::At(value) => format!("at:{}", value.target_id),
		puniyu_element::receive::Elements::Reply(value) => format!("reply:{}", value.message_id),
		puniyu_element::receive::Elements::Face(value) => format!("face:{}", value.id),
		puniyu_element::receive::Elements::Image(value) => {
			format!("image:{}", value.summary.as_deref().unwrap_or(value.file_name.as_str()))
		}
		puniyu_element::receive::Elements::File(value) => format!("file:{}", value.file_name),
		puniyu_element::receive::Elements::Video(value) => format!("video:{}", value.file_name),
		puniyu_element::receive::Elements::Record(value) => format!("record:{}", value.file_name),
		puniyu_element::receive::Elements::Json(value) => format!("json:{}", value.data),
		puniyu_element::receive::Elements::Xml(value) => format!("xml:{}", value.data),
	}
}
