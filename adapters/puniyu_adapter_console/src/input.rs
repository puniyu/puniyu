use puniyu_adapter::contact::SceneType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConsolePayload {
	At(String),
	Text(String),
	Image(String),
	Json(String),
	Video(String),
	Record(String),
	File(String),
	Xml(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedConsoleInput {
	pub scene: SceneType,
	pub payload: ConsolePayload,
}

pub fn parse_console_input(line: &str) -> ParsedConsoleInput {
	let (scene, content) = match line.split_once(':') {
		Some(("group", rest)) => (SceneType::Group, rest),
		Some(("grouptemp", rest)) => (SceneType::GroupTemp, rest),
		Some(("friend", rest)) => (SceneType::Friend, rest),
		Some(("guild", rest)) => (SceneType::Guild, rest),
		_ => (SceneType::Friend, line),
	};

	let payload = match content.split_once(':') {
		Some(("at", target_id)) if !target_id.is_empty() => {
			ConsolePayload::At(target_id.to_string())
		}
		Some(("text", text_content)) => ConsolePayload::Text(text_content.to_string()),
		Some(("image", image_url)) => ConsolePayload::Image(image_url.to_string()),
		Some(("json", json_content)) => ConsolePayload::Json(json_content.to_string()),
		Some(("video", video_url)) => ConsolePayload::Video(video_url.to_string()),
		Some(("record", record_url)) => ConsolePayload::Record(record_url.to_string()),
		Some(("file", file_url)) => ConsolePayload::File(file_url.to_string()),
		Some(("xml", xml_content)) => ConsolePayload::Xml(xml_content.to_string()),
		_ => ConsolePayload::Text(content.to_string()),
	};

	ParsedConsoleInput { scene, payload }
}
