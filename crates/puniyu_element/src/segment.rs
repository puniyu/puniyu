use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Segment {
	#[serde(rename = "type")]
	pub r#type: String,
	pub data: Value,
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
	// 文本元素
	(text, $text:expr) => {
		Segment {
			r#type: ElementType::Text.to_string(),
			data: serde_json::json!(
				{
					"text": $text
				}
			),
		}
	};
}
