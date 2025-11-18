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
	// 图片元素
	(image, $file:expr) => {
		Segment {
			r#type: ElementType::Image.to_string(),
			data: serde_json::json!(
				{
					"file": $file
				}
			),
		}
	};
	// 回复元素
	(reply, $message_id:expr) => {
		Segment {
			r#type: ElementType::Reply.to_string(),
			data: serde_json::json!(
				{
					"message_id": $message_id
				}
			),
		}
	};
	// 文件元素
	(file, $file:expr) => {
		Segment {
			r#type: ElementType::File.to_string(),
			data: serde_json::json!(
				{
					"file": $file
				}
			),
		}
	};
	// 语音元素
	(recording, $file:expr) => {
		Segment {
			r#type: ElementType::Recording.to_string(),
			data: serde_json::json!(
				{
					"file": $file
				}
			),
		}
	};
	// 视频元素
	(video, $file:expr) => {
		Segment {
			r#type: ElementType::Video.to_string(),
			data: serde_json::json!(
				{
					"file": $file
				}
			),
		}
	};
	// 表情元素
	(face, $id:expr) => {
		Segment {
			r#type: ElementType::Face.to_string(),
			data: serde_json::json!(
				{
					"id": $id
				}
			),
		}
	};
	// json元素
	(json, $json:expr) => {
		Segment {
			r#type: ElementType::Json.to_string(),
			data: serde_json::json!(
				{
					"json": $json
				}
			),
		}
	};
	// xml元素
	(xml, $xml:expr) => {
		Segment {
			r#type: ElementType::Xml.to_string(),
			data: serde_json::json!(
				{
					"xml": $xml
				}
			),
		}
	};
}
