pub mod element;
mod segment;

use crate::message::element::receive::{
    ReceiveAtElement, ReceiveElements, ReceiveImageElement, ReceiveVideoElement,
};
use crate::message::element::send::{
    JsonElement, MusicElement, RecordElement, ReplyElement, TextElement, XmlElement,
};
pub use segment::{music, send::Segment};

pub struct RawMessage {
    pub raw: String,
    pub msg: String,
}

impl RawMessage {
    pub fn new(raw: String, msg: String) -> Self {
        Self { raw, msg }
    }
}

pub fn create_raw_message(data: Vec<ReceiveElements>) -> RawMessage {
    let mut msg_parts: Vec<String> = Vec::new();
    let raw_str_parts: Vec<String> = data
        .into_iter()
        .map(|element| match element {
            ReceiveElements::Text(TextElement { text, .. }) => {
                msg_parts.push(text.clone());
                text
            }
            ReceiveElements::Image(ReceiveImageElement { file, .. }) => {
                let formatted_file = format_file_display(&file);
                msg_parts.push(format!("[图片:{}]", formatted_file));
                format!("[图片:{}]", formatted_file)
            }
            ReceiveElements::At(ReceiveAtElement { target_id, .. }) => {
                let at_text = format!("[at:{}]", target_id);
                msg_parts.push(at_text.clone());
                at_text
            }
            ReceiveElements::Json(JsonElement { data, .. }) => {
                msg_parts.push(data.clone());
                format!("[json:{}]", data)
            }
            ReceiveElements::Music(MusicElement { .. }) => {
                let music_text = "[music]".to_string();
                msg_parts.push(music_text.clone());
                music_text
            }
            ReceiveElements::Record(RecordElement { file, .. }) => {
                let formatted_file = format_file_display(&file);
                let record_text = format!("[record:{}]", formatted_file);
                msg_parts.push(record_text.clone());
                record_text
            }
            ReceiveElements::Reply(ReplyElement { message_id, .. }) => {
                let reply_text = format!("[reply:{}]", message_id);
                msg_parts.push(reply_text.clone());
                reply_text
            }
            ReceiveElements::Video(ReceiveVideoElement { file, .. }) => {
                let formatted_file = format_file_display(&file);
                let video_text = format!("[video:{}]", formatted_file);
                msg_parts.push(video_text.clone());
                video_text
            }
            ReceiveElements::Xml(XmlElement { data, .. }) => {
                msg_parts.push(data.clone());
                format!("[xml:{}]", data)
            }
        })
        .collect();

    RawMessage {
        raw: raw_str_parts.join(""),
        msg: msg_parts.join(""),
    }
}

fn format_file_display(file: &str) -> String {
    if file.starts_with("http") || file.starts_with("file") {
        file.to_string()
    } else if file.starts_with("base64://") {
        "base64://...".to_string()
    } else {
        "file://...".to_string()
    }
}
