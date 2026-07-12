use puniyu_element::send::{AtElement, Elements, FaceElement, FileElement, ImageElement, JsonElement, RecordElement, ReplyElement, TextElement, VideoElement, XmlElement};
use puniyu_element::File;
use puniyu_message::message;

#[test]
fn test_message_macro_single() {
	let msg = message!(Elements::Text(TextElement::new("hello")));
	assert_eq!(msg.len(), 1);
}

#[test]
fn test_message_macro_multiple() {
	let msg = message!(
		Elements::Text(TextElement::new("hello")),
		Elements::At(AtElement::new("123456"))
	);
	assert_eq!(msg.len(), 2);
}

#[test]
fn test_message_macro_trailing_comma() {
	let msg = message!(
		Elements::Text(TextElement::new("hi")),
	);
	assert_eq!(msg.len(), 1);
}

#[test]
fn test_message_macro_empty() {
	let msg = message!();
	assert!(msg.is_empty());
}

#[test]
fn test_from_single_element() {
	let elem = Elements::Text(TextElement::new("hello"));
	let msg: puniyu_message::Message = elem.into();
	assert_eq!(msg.len(), 1);
}

#[test]
fn test_from_vec_elements() {
	let vec = vec![
		Elements::Text(TextElement::new("hello")),
		Elements::Text(TextElement::new("world")),
	];
	let msg: puniyu_message::Message = vec.into();
	assert_eq!(msg.len(), 2);
}

#[test]
fn test_from_str() {
	let msg: puniyu_message::Message = "hello".into();
	assert_eq!(msg.len(), 1);
	let elem = &msg[0];
	assert!(matches!(elem, Elements::Text(_)));
}

#[test]
fn test_from_string() {
	let msg: puniyu_message::Message = String::from("hello").into();
	assert_eq!(msg.len(), 1);
}

#[test]
fn test_into_vec() {
	let msg = message!(Elements::Text(TextElement::new("test")));
	let vec: Vec<Elements> = msg.into();
	assert_eq!(vec.len(), 1);
}

#[test]
fn test_deref() {
	let msg = message!(Elements::Text(TextElement::new("hi")));
	let slice: &[Elements] = &msg;
	assert_eq!(slice.len(), 1);
	assert!(!msg.is_empty());
	assert_eq!(msg.len(), 1);
}

#[test]
fn test_iter() {
	let msg = message!(
		Elements::Text(TextElement::new("a")),
		Elements::Text(TextElement::new("b")),
	);
	let count = msg.iter().count();
	assert_eq!(count, 2);
}

#[test]
fn test_serde_roundtrip() {
	let msg = message!(
		Elements::Text(TextElement::new("hello")),
		Elements::At(AtElement::new("123456")),
	);
	let json = serde_json::to_string(&msg).expect("serialize");
	let restored: puniyu_message::Message = serde_json::from_str(&json).expect("deserialize");
	assert_eq!(msg, restored);
}


#[test]
fn test_display_alternate() {
	let msg = message!(
		Elements::Text(TextElement::new("hello")),
		Elements::Text(TextElement::new("world")),
	);
	assert_eq!(format!("{:#}", msg), "Text\nText");
}


#[test]
fn test_all_element_types() {
	let msg = message!(
		Elements::Text(TextElement::new("t")),
		Elements::At(AtElement::new("u1")),
		Elements::Reply(ReplyElement::new("r1")),
		Elements::Face(FaceElement::new(1u64)),
		Elements::Image(ImageElement::new(File::Bytes(bytes::Bytes::from_static(b"png")), "i.png", None::<&str>)),
		Elements::File(FileElement::new(File::Bytes(bytes::Bytes::from_static(b"bin")), "f.bin")),
		Elements::Video(VideoElement::new(File::Bytes(bytes::Bytes::from_static(b"mp4")), "v.mp4")),
		Elements::Record(RecordElement::new(File::Bytes(bytes::Bytes::from_static(b"silk")), "a.silk")),
		Elements::Json(JsonElement::new("{}")),
		Elements::Xml(XmlElement::new("<r/>")),
	);
	assert_eq!(msg.len(), 10);
}
