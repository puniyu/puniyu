use super::RawMessage;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Segment {
	#[serde(rename = "type")]
	pub r#type: String,
	pub data: Value,
}

impl fmt::Display for Segment {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if f.alternate() {
			// 使用 {:#} 格式时，显示美化的 JSON
			match serde_json::to_string_pretty(self) {
				Ok(json) => write!(f, "{}", json),
				Err(_) => write!(f, "Segment {{ type: {}, data: {} }}", self.r#type, self.data),
			}
		} else {
			// 使用 {} 格式时，显示紧凑的 JSON
			match serde_json::to_string(self) {
				Ok(json) => write!(f, "{}", json),
				Err(_) => write!(f, "Segment {{ type: {}, data: {} }}", self.r#type, self.data),
			}
		}
	}
}

impl RawMessage for Segment {
	fn raw(&self) -> String {
		match self.r#type.as_str() {
			"at" => format!("[at:{}]", self.data),
			"text" => format!("[text:{}]", self.data),
			"image" => format!("[image:{}]", self.data),
			"video" => format!("[video:{}]", self.data),
			"record" => format!("[record:{}]", self.data),
			"reply" => format!("[reply:{}]", self.data),
			"file" => format!("[file:{}]", self.data),
			_ => format!("[unkonwn:{}]", self.data),
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
