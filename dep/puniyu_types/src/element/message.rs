use super::ElementType;
use super::segment::Segment;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message(Vec<Segment>);

impl From<Vec<Segment>> for Message {
	fn from(message: Vec<Segment>) -> Self {
		Message(message)
	}
}

impl From<Segment> for Message {
	fn from(segment: Segment) -> Self {
		Message(vec![segment])
	}
}

impl From<Message> for Vec<Segment> {
	fn from(message: Message) -> Self {
		message.0
	}
}

impl From<&str> for Message {
	fn from(v: &str) -> Self {
		Message(vec![Segment {
			r#type: ElementType::Text.to_string(),
			data: json!({
				"text":v,
			}),
		}])
	}
}

impl From<String> for Message {
	fn from(v: String) -> Self {
		Message(vec![Segment {
			r#type: ElementType::Text.to_string(),
			data: json!({
				"text":v,
			}),
		}])
	}
}

impl From<&String> for Message {
	fn from(v: &String) -> Self {
		Message(vec![Segment {
			r#type: ElementType::Text.to_string(),
			data: json!({
				"text":v,
			}),
		}])
	}
}
