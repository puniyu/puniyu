use std::str::FromStr;

use puniyu_element::receive::{
	AtElement as RecvAtElement, FaceElement as RecvFaceElement, FileElement as RecvFileElement,
	ImageElement as RecvImageElement, JsonElement as RecvJsonElement,
	RecordElement as RecvRecordElement, ReplyElement as RecvReplyElement,
	TextElement as RecvTextElement, VideoElement as RecvVideoElement, XmlElement as RecvXmlElement,
};
use puniyu_element::send::{
	AtElement, FaceElement, FileElement, ImageElement, JsonElement, RecordElement, ReplyElement,
	TextElement, VideoElement, XmlElement,
};
use puniyu_element::{Element, ElementType, File, receive, send};

#[test]
fn test_element_type_str_roundtrip() {
	let cases = [
		(ElementType::At, "at"),
		(ElementType::Reply, "reply"),
		(ElementType::Text, "text"),
		(ElementType::Image, "image"),
		(ElementType::File, "file"),
		(ElementType::Record, "record"),
		(ElementType::Video, "video"),
		(ElementType::Face, "face"),
		(ElementType::Json, "json"),
		(ElementType::Xml, "xml"),
	];
	for (value, text) in cases {
		assert_eq!(value.to_string(), text);
		assert_eq!(ElementType::from_str(text).expect("parse"), value);
	}
}

#[test]
fn test_element_type_json_roundtrip() {
	let cases = [
		(ElementType::At, r#""at""#),
		(ElementType::Reply, r#""reply""#),
		(ElementType::Text, r#""text""#),
		(ElementType::Image, r#""image""#),
		(ElementType::File, r#""file""#),
		(ElementType::Record, r#""record""#),
		(ElementType::Video, r#""video""#),
		(ElementType::Face, r#""face""#),
		(ElementType::Json, r#""json""#),
		(ElementType::Xml, r#""xml""#),
	];
	for (value, expected_json) in cases {
		let json = serde_json::to_string(&value).expect("serialize");
		assert_eq!(json, expected_json);
		let decoded: ElementType = serde_json::from_str(&json).expect("deserialize");
		assert_eq!(decoded, value);
	}
}

#[test]
fn test_send_each_variant_constructs_and_serializes() {
	let cases: Vec<send::Elements> = vec![
		send::Elements::from(TextElement::new("hello")),
		send::Elements::from(AtElement::new("u1")),
		send::Elements::from(ReplyElement::new("m1")),
		send::Elements::from(FaceElement::new(99u64)),
		send::Elements::from(FileElement::new(bytes::Bytes::from_static(b"x"), "x.bin")),
		send::Elements::from(ImageElement::new(
			bytes::Bytes::from_static(b"\x89PNG"),
			"i.png",
			None::<&str>,
		)),
		send::Elements::from(VideoElement::new(bytes::Bytes::from_static(b"v"), "v.mp4")),
		send::Elements::from(RecordElement::new(bytes::Bytes::from_static(b"a"), "a.silk")),
		send::Elements::from(JsonElement::new("{}")),
		send::Elements::from(XmlElement::new("<r/>")),
	];

	let expected_types = [
		ElementType::Text,
		ElementType::At,
		ElementType::Reply,
		ElementType::Face,
		ElementType::File,
		ElementType::Image,
		ElementType::Video,
		ElementType::Record,
		ElementType::Json,
		ElementType::Xml,
	];

	for (element, expected) in cases.iter().zip(expected_types.iter()) {
		assert_eq!(element.r#type(), *expected);
		let json = serde_json::to_string(element).expect("serialize");
		let restored: send::Elements = serde_json::from_str(&json).expect("deserialize");
		assert_eq!(restored.r#type(), *expected);
	}
}

#[test]
fn test_receive_each_variant_constructs_and_serializes() {
	let text: RecvTextElement = "hi".into();
	let at: RecvAtElement = "u1".into();
	let reply: RecvReplyElement = "m1".into();
	let face = RecvFaceElement { id: 1 };
	let file: RecvFileElement = RecvFileElement {
		file: File::Bytes(bytes::Bytes::from_static(b"x")),
		file_size: 1,
		file_name: "x.bin".into(),
	};
	let image: RecvImageElement = RecvImageElement {
		file: File::Bytes(bytes::Bytes::from_static(b"x")),
		file_name: "i.png".into(),
		summary: Some("s".into()),
		width: 1,
		height: 1,
	};
	let video: RecvVideoElement = RecvVideoElement {
		file: File::Bytes(bytes::Bytes::from_static(b"v")),
		file_name: "v.mp4".into(),
	};
	let record: RecvRecordElement = RecvRecordElement {
		file: File::Bytes(bytes::Bytes::from_static(b"a")),
		file_name: "a.silk".into(),
	};
	let json_e: RecvJsonElement = "{}".into();
	let xml: RecvXmlElement = "<r/>".into();

	let cases: Vec<receive::Elements> = vec![
		receive::Elements::Text(text),
		receive::Elements::At(at),
		receive::Elements::Reply(reply),
		receive::Elements::Face(face),
		receive::Elements::File(file),
		receive::Elements::Image(image),
		receive::Elements::Video(video),
		receive::Elements::Record(record),
		receive::Elements::Json(json_e),
		receive::Elements::Xml(xml),
	];

	let expected_types = [
		ElementType::Text,
		ElementType::At,
		ElementType::Reply,
		ElementType::Face,
		ElementType::File,
		ElementType::Image,
		ElementType::Video,
		ElementType::Record,
		ElementType::Json,
		ElementType::Xml,
	];

	for (element, expected) in cases.iter().zip(expected_types.iter()) {
		assert_eq!(element.r#type(), *expected);
		let json = serde_json::to_string(element).expect("serialize");
		let restored: receive::Elements = serde_json::from_str(&json).expect("deserialize");
		assert_eq!(restored.r#type(), *expected);
	}
}

#[test]
fn test_at_everyone_via_send_and_receive() {
	let send_everyone = send::Elements::from(AtElement::everyone());
	assert_eq!(send_everyone.r#type(), ElementType::At);

	let at: RecvAtElement = "all".into();
	let recv_everyone = receive::Elements::from(at);
	assert!(recv_everyone.as_at().expect("at").is_everyone());
}

#[test]
fn test_send_serde_uses_tagged_format() {
	let text = send::Elements::from(TextElement::new("hi"));
	let json = serde_json::to_string(&text).expect("serialize");
	assert!(json.contains("\"type\":\"text\""));
	assert!(json.contains("\"field0\""));
}

#[test]
fn test_receive_serde_uses_tagged_format() {
	let at: RecvAtElement = "u1".into();
	let elements = receive::Elements::from(at);
	let json = serde_json::to_string(&elements).expect("serialize");
	assert!(json.contains("\"type\":\"at\""));
	assert!(json.contains("\"field0\""));
}
