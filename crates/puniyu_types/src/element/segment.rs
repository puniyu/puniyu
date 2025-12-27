#[cfg(feature = "element")]
#[macro_export]
macro_rules! segment {
	// at元素
	(at, $target_id:expr) => {
		Elements::At(AtElement { target_id: $target_id.to_string() })
	};
	(at_all) => {
		Elements::At(AtElement { target_id: String::from("all") })
	};
	// 文本元素
	(text, $text:expr) => {
		Elements::Text(TextElement { text: $text.to_string() })
	};
	// 图片元素
	(image, $file:expr) => {
		Elements::Image(ImageElement { file: $file, is_flash: false, summary: None })
	};
	(image, $file:expr, $is_flash:expr) => {
		Elements::Image(ImageElement { file: $file, is_flash: $is_flash, summary: None })
	};
	(image, $file:expr, $is_flash:expr, $summary:expr) => {
		Elements::Image(ImageElement { file: $file, is_flash: $is_flash, summary: $summary })
	};
	// 回复元素
	(reply, $message_id:expr) => {
		Elements::Reply(ReplyElement { message_id: $message_id })
	};
	// 文件元素
	(file, $file:expr) => {
		Elements::File(FileElement { file: $file })
	};
	// 语音元素
	(record, $file:expr) => {
		Elements::Record(RecordElement { file: $file })
	};
	// 视频元素
	(video, $file:expr) => {
		Elements::Video(VideoElement { file: $file })
	};
	// 表情元素
	(face, $id:expr) => {
		Elements::Face(FaceElement { id: $id })
	};
	// json元素
	(json, $json:expr) => {
		Elements::Json(JsonElement { data: $json.to_string() })
	};
	// xml元素
	(xml, $xml:expr) => {
		Elements::Xml(XmlElement { file: $xml.to_string() })
	};
}

#[cfg(feature = "event")]
#[macro_export]
macro_rules! message {
	// 空消息
	() => {
		$crate::element::Message::from(Vec::<$crate::element::send::Elements>::new())
	};
	// 单个字符串字面量
	($text:literal) => {
		$crate::element::Message::from($text)
	};
	// 单个 Elements
	($elem:expr) => {
		$crate::element::Message::from($elem)
	};
	// 多个 Elements
	($($elem:expr),+ $(,)?) => {
		$crate::element::Message::from(vec![$($elem),+])
	};
}
