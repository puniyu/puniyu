use puniyu_element::File;
use puniyu_element::send::{
	AtElement, Elements, FaceElement, FileElement, ImageElement, JsonElement, RecordElement,
	ReplyElement, TextElement, VideoElement, XmlElement,
};
use puniyu_message::{Message, message};

#[test]
fn test_message_macro_single() {
	let msg = message!(TextElement::new("hello"));
	assert_eq!(msg.len(), 1);
}

#[test]
fn test_message_macro_multiple() {
	let msg = message!(TextElement::new("hello"), AtElement::new("123456"));
	assert_eq!(msg.len(), 2);
}

#[test]
fn test_message_macro_trailing_comma() {
	let msg = message!(TextElement::new("hi"),);
	assert_eq!(msg.len(), 1);
}

#[test]
fn test_message_macro_empty() {
	let msg = message!();
	assert!(msg.is_empty());
}

#[test]
fn test_message_macro_mixed_elements() {
	let msg = message!(TextElement::new("hello"), Elements::At(AtElement::new("user-1")));
	assert!(matches!(&msg[0], Elements::Text(_)));
	assert!(matches!(&msg[1], Elements::At(_)));
}

#[test]
fn test_builder_all_domain_methods() {
	let msg = Message::builder()
		.text("text")
		.at("user-1")
		.at_everyone()
		.reply("message-1")
		.face(1)
		.image(bytes::Bytes::from_static(b"image"), "image.png", Some("summary"))
		.file(bytes::Bytes::from_static(b"file"), "file.bin")
		.video(bytes::Bytes::from_static(b"video"), "video.mp4")
		.record(bytes::Bytes::from_static(b"record"), "record.silk")
		.json("{}")
		.xml("<root/>")
		.build();

	assert!(matches!(&msg[0], Elements::Text(_)));
	assert!(matches!(&msg[1], Elements::At(at) if at.target_id == "user-1"));
	assert!(matches!(&msg[2], Elements::At(at) if at.target_id == "all"));
	assert!(matches!(&msg[3], Elements::Reply(_)));
	assert!(matches!(&msg[4], Elements::Face(_)));
	assert!(matches!(&msg[5], Elements::Image(_)));
	assert!(matches!(&msg[6], Elements::File(_)));
	assert!(matches!(&msg[7], Elements::Video(_)));
	assert!(matches!(&msg[8], Elements::Record(_)));
	assert!(matches!(&msg[9], Elements::Json(_)));
	assert!(matches!(&msg[10], Elements::Xml(_)));
}

#[test]
fn test_builder_element_chaining() {
	let image = ImageElement::builder()
		.file(bytes::Bytes::from_static(b"image"))
		.file_name("image.png")
		.build();
	let msg = Message::builder()
		.element(image)
		.element(TextElement::new("a"))
		.element(TextElement::new("b"))
		.build();

	assert!(matches!(&msg[0], Elements::Image(image) if image.summary.is_none()));
	assert_eq!(msg[1].as_text(), Some("a"));
	assert_eq!(msg[2].as_text(), Some("b"));
}

#[test]
fn test_from_single_element() {
	let elem = Elements::Text(TextElement::new("hello"));
	let msg: puniyu_message::Message = elem.into();
	assert_eq!(msg.len(), 1);
}

#[test]
fn test_from_vec_elements() {
	let vec =
		vec![Elements::Text(TextElement::new("hello")), Elements::Text(TextElement::new("world"))];
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
	let msg =
		message!(Elements::Text(TextElement::new("a")), Elements::Text(TextElement::new("b")),);
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
	assert!(json.starts_with('['));
	let restored: puniyu_message::Message = serde_json::from_str(&json).expect("deserialize");
	assert_eq!(msg, restored);
}

#[test]
fn test_collection_conversions_and_iteration() {
	let collected: Message = [TextElement::new("a"), TextElement::new("b")].into_iter().collect();
	assert_eq!(collected.as_slice().len(), 2);
	assert_eq!((&collected).into_iter().count(), 2);

	let elements: ecow::EcoVec<Elements> = collected.clone().into_elements();
	let message = Message::new(elements.clone());
	assert_eq!(message.as_slice(), elements.as_slice());

	let restored: ecow::EcoVec<Elements> = message.clone().into();
	assert_eq!(restored, elements);
	assert_eq!(message.into_iter().count(), 2);
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
		Elements::Image(ImageElement::new(
			File::Bytes(bytes::Bytes::from_static(b"png")),
			"i.png",
			None::<&str>
		)),
		Elements::File(FileElement::new(File::Bytes(bytes::Bytes::from_static(b"bin")), "f.bin")),
		Elements::Video(VideoElement::new(File::Bytes(bytes::Bytes::from_static(b"mp4")), "v.mp4")),
		Elements::Record(RecordElement::new(
			File::Bytes(bytes::Bytes::from_static(b"silk")),
			"a.silk"
		)),
		Elements::Json(JsonElement::new("{}")),
		Elements::Xml(XmlElement::new("<r/>")),
	);
	assert_eq!(msg.len(), 10);
}
