use super::ElementType;
use super::segment::Segment;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message(Vec<Segment>);

impl fmt::Display for Message {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if f.alternate() {
			let segments: Vec<String> = self.0.iter().map(|s| format!("{:#}", s)).collect();
			write!(f, "{}", segments.join("\n"))
		} else {
			let segments: Vec<String> = self.0.iter().map(|s| s.to_string()).collect();
			write!(f, "{}", segments.join(""))
		}
	}
}

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
