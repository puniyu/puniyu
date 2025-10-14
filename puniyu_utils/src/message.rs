pub mod element;
mod segment;

use crate::message::element::ElementType;
pub use segment::{Segment, music};
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
