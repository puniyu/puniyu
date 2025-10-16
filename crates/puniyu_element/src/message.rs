use crate::segment::Segment;
use crate::{ElementType, RawMessage};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message(Vec<Segment>);

impl From<Vec<Segment>> for Message {
	fn from(v: Vec<Segment>) -> Self {
		Message(v)
	}
}

impl From<Segment> for Message {
	fn from(segment: Segment) -> Self {
		Message(vec![segment])
	}
}

impl From<Message> for Vec<Segment> {
	fn from(v: Message) -> Self {
		v.0
	}
}

impl RawMessage for Message {
	fn raw(&self) -> String {
		for segment in &self.0 {
			match segment.r#type.as_str() {
				t if t == ElementType::Text.to_string() => {
					return format!("[text:{}]", segment.data.get("text").unwrap());
				}
				t if t == ElementType::Image.to_string() => {
					return format!("[image:{}]", segment.data.get("file").unwrap());
				}
				_ => continue,
			}
		}
		String::new()
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
