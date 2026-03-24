use puniyu_element::send::*;
use puniyu_element::{Element, ElementType};
use puniyu_element::RawMessage;

#[test]
fn test_send_text_element_new() {
	let element = TextElement::new("send text");

	assert_eq!(element.text, "send text");
}

#[test]
fn test_send_text_element_raw_message() {
	let element = TextElement::new("test");

	assert_eq!(element.r#type(), ElementType::Text);
	assert_eq!(element.raw(), "test");
}

#[test]
fn test_send_at_element_new() {
	let element = AtElement::new("user123");

	assert_eq!(element.target_id, "user123");
}

#[test]
fn test_send_at_element_raw_message() {
	let element = AtElement::new("target");

	assert_eq!(element.r#type(), ElementType::At);
	assert_eq!(element.raw(), "target");
}

#[test]
fn test_send_reply_element_new() {
	let element = ReplyElement::new("msg_id");

	assert_eq!(element.message_id, "msg_id");
}

#[test]
fn test_send_reply_element_raw_message() {
	let element = ReplyElement::new("reply_to");

	assert_eq!(element.r#type(), ElementType::Reply);
	assert_eq!(element.raw(), "reply_to");
}

#[test]
fn test_send_face_element_new() {
	let element = FaceElement::new(5u64);

	assert_eq!(element.id, 5);
}

#[test]
fn test_send_face_element_raw_message() {
	let element = FaceElement::new(10u64);

	assert_eq!(element.r#type(), ElementType::Face);
	assert_eq!(element.raw(), "10");
}

#[test]
fn test_send_face_element_from_u32() {
	let element = FaceElement::new(42u32);

	assert_eq!(element.id, 42);
}

#[test]
fn test_send_image_element_new() {
	let file_data = b"fake image data".to_vec();
	let element = ImageElement::new(file_data.clone(), "beautiful scenery", None);

	assert_eq!(element.file.as_ref(), file_data.as_slice());
	assert_eq!(element.summary, "beautiful scenery");
}

#[test]
fn test_send_image_element_raw_message() {
	let file_data = b"test image".to_vec();
	let element = ImageElement::new(file_data, "test image",  None);

	assert_eq!(element.r#type(), ElementType::Image);
	assert_eq!(element.raw(), "test image");
}

#[test]
fn test_send_file_element_new() {
	let file_data = b"file content".to_vec();
	let element = FileElement::new(file_data.clone(), "document.pdf");

	assert_eq!(element.file.as_ref(), file_data.as_slice());
	assert_eq!(element.file_name, "document.pdf");
}

#[test]
fn test_send_file_element_raw_message() {
	let file_data = b"test file".to_vec();
	let element = FileElement::new(file_data, "file.txt");

	assert_eq!(element.r#type(), ElementType::File);
	assert_eq!(element.raw(), "file.txt");
}

#[test]
fn test_send_video_element_new() {
	let video_data = b"video content".to_vec();
	let element = VideoElement::new(video_data.clone(), "movie.mp4");

	assert_eq!(element.file.as_ref(), video_data.as_slice());
	assert_eq!(element.file_name, "movie.mp4");
}

#[test]
fn test_send_video_element_raw_message() {
	let video_data = b"test video".to_vec();
	let element = VideoElement::new(video_data, "clip.mp4");

	assert_eq!(element.r#type(), ElementType::Video);
	assert_eq!(element.raw(), "[video:clip.mp4]");
}

#[test]
fn test_send_record_element_new() {
	let audio_data = b"audio content".to_vec();
	let element = RecordElement::new(audio_data.clone(), "voice.mp3");

	assert_eq!(element.file.as_ref(), audio_data.as_slice());
	assert_eq!(element.file_name, "voice.mp3");
}

#[test]
fn test_send_record_element_raw_message() {
	let audio_data = b"test audio".to_vec();
	let element = RecordElement::new(audio_data, "sound.mp3");

	assert_eq!(element.r#type(), ElementType::Record);
	assert_eq!(element.raw(), "sound.mp3");
}

#[test]
fn test_send_json_element_new() {
	let json = r#"{"status": "ok"}"#;
	let element = JsonElement::new(json);

	assert_eq!(element.data, json);
}

#[test]
fn test_send_json_element_raw_message() {
	let element = JsonElement::new("{}");

	assert_eq!(element.r#type(), ElementType::Json);
	assert_eq!(element.raw(), "{}");
}

#[test]
fn test_send_xml_element_new() {
	let xml = "<root><item>test</item></root>";
	let element = XmlElement::new(xml);

	assert_eq!(element.data, xml);
}

#[test]
fn test_send_xml_element_raw_message() {
	let element = XmlElement::new("<xml/>");

	assert_eq!(element.r#type(), ElementType::Xml);
	assert_eq!(element.raw(), "<xml/>");
}

#[test]
fn test_send_elements_enum_text() {
	let text_elem = TextElement::new("message");
	let element = Elements::Text(text_elem);

	assert_eq!(element.as_text(), Some("message"));
	assert!(element.as_at().is_none());
}

#[test]
fn test_send_elements_enum_at() {
	let at_elem = AtElement::new("user");
	let element = Elements::At(at_elem);

	assert!(element.as_text().is_none());
	assert!(element.as_at().is_some());
	assert_eq!(element.as_at().unwrap().target_id, "user");
}

#[test]
fn test_send_elements_enum_reply() {
	let reply_elem = ReplyElement::new("msg");
	let element = Elements::Reply(reply_elem);

	assert!(element.as_reply().is_some());
	assert_eq!(element.as_reply().unwrap().message_id, "msg");
}

#[test]
fn test_send_elements_enum_face() {
	let face_elem = FaceElement::new(3u64);
	let element = Elements::Face(face_elem);

	assert!(element.as_face().is_some());
	assert_eq!(element.as_face().unwrap().id, 3);
}

#[test]
fn test_send_elements_enum_image() {
	let image_data = b"image".to_vec();
	let image_elem = ImageElement::new(image_data, "pic.jpg", None);
	let element = Elements::Image(image_elem);

	assert!(element.as_image().is_some());
	assert_eq!(element.as_image().unwrap().summary, "pic.jpg");
}

#[test]
fn test_send_elements_enum_file() {
	let file_data = b"data".to_vec();
	let file_elem = FileElement::new(file_data, "data.txt");
	let element = Elements::File(file_elem);

	assert!(element.as_file().is_some());
	assert_eq!(element.as_file().unwrap().file_name, "data.txt");
}

#[test]
fn test_send_elements_enum_video() {
	let video_data = b"video".to_vec();
	let video_elem = VideoElement::new(video_data, "clip.mp4");
	let element = Elements::Video(video_elem);

	assert!(element.as_video().is_some());
	assert_eq!(element.as_video().unwrap().file_name, "clip.mp4");
}

#[test]
fn test_send_elements_enum_record() {
	let audio_data = b"audio".to_vec();
	let record_elem = RecordElement::new(audio_data, "voice.mp3");
	let element = Elements::Record(record_elem);

	assert!(element.as_record().is_some());
	assert_eq!(element.as_record().unwrap().file_name, "voice.mp3");
}

#[test]
fn test_send_elements_enum_json() {
	let json_elem = JsonElement::new("{}");
	let element = Elements::Json(json_elem);

	assert!(element.as_json().is_some());
	assert_eq!(element.as_json().unwrap().data, "{}");
}

#[test]
fn test_send_elements_enum_xml() {
	let xml_elem = XmlElement::new("<xml/>");
	let element = Elements::Xml(xml_elem);

	assert!(element.as_xml().is_some());
	assert_eq!(element.as_xml().unwrap().data, "<xml/>");
}

#[test]
fn test_send_elements_display() {
	let text_elem = TextElement::new("display test");
	let element = Elements::Text(text_elem);

	// 测试 Display trait
	let display_str = format!("{}", element);
	assert!(!display_str.is_empty());
}

#[test]
fn test_send_bytes_conversion() {
	let data = vec![1, 2, 3, 4, 5];
	let element = FileElement::new(data.clone(), "test.bin");

	assert_eq!(element.file.len(), 5);
	assert_eq!(element.file.as_ref(), data.as_slice());
}

#[test]
fn test_send_empty_bytes() {
	let empty_data: Vec<u8> = vec![];
	let element = ImageElement::new(empty_data, "empty.png", None);

	assert_eq!(element.file.len(), 0);
}
