pub mod music;

use crate::message::element::ElementType;
pub use crate::message::element::send::{
	FileElement, ImageElement, JsonElement, MusicElement, RecordElement, ReplyElement, TextElement,
	VideoElement, XmlElement,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Segment {
	#[serde(rename = "type")]
	pub r#type: String,
	pub data: Value,
}

impl Segment {
	pub fn at(target_id: &str) -> Self {
		Segment {
			r#type: ElementType::At.to_string(),
			data: serde_json::json!({
				"target_id": target_id.to_string()
			}),
		}
	}
}

#[macro_export]
macro_rules! segment {
	// at元素
	(at, $target_id:expr) => {
		Segment {
			r#type: ElementType::At.to_string(),
			data: serde_json::json!(
				{
					"target_id": $target_id
				}
			),
		}
	};
}
