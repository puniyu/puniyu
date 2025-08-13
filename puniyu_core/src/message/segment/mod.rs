use serde::{Deserialize, Serialize};

mod at_element;
pub use at_element::AtElement as at;
mod face_element;
pub use face_element::FaceElement as face;
mod file_element;
pub use file_element::FileElement as file;
mod image_element;
pub use image_element::ImageElement as image;
mod json_element;
pub use json_element::JsonElement as json;
mod music_element;
pub use music_element::MusicElement as music;
mod record_element;
pub use record_element::RecordElement as record;
mod reply_element;
pub use reply_element::ReplyElement as reply;
mod text_element;
pub use text_element::TextElement as text;
mod video_element;
pub use video_element::VideoElement as video;
mod xml_element;
pub use xml_element::XmlElement as xml;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 消息元素
pub enum Element {
    At(at_element::AtElement),
    Face(face_element::FaceElement),
    File(file_element::FileElement),
    Image(image_element::ImageElement),
    Json(json_element::JsonElement),
    Music(music_element::MusicElement),
    Record(record_element::RecordElement),
    Reply(reply_element::ReplyElement),
    Text(text_element::TextElement),
    Video(video_element::VideoElement),
    Xml(xml_element::XmlElement),
}
