use super::music::MusicData;
use crate::message::element::send::{
    AtElement, FaceElement, FileElement, ImageElement, JsonElement, MusicElement, RecordElement,
    ReplyElement, TextElement, VideoElement, XmlElement,
};

pub struct Segment {
    /// at元素
    pub at: AtElement,
    /// 表情元素
    pub face: FaceElement,
    /// 文件元素
    pub file: FileElement,
    /// 图片元素
    pub image: ImageElement,
    /// json元素
    pub json: JsonElement,
    /// 语音元素
    pub record: RecordElement,
    /// 回复元素
    pub reply: ReplyElement,
    /// 文本元素
    pub text: TextElement,
    /// 视频元素
    pub video: VideoElement,
    /// xml元素
    pub xml: XmlElement,
}
impl Segment {
    /// 创建一个at元素
    ///
    /// # 参数
    ///
    /// * `target_id`
    ///     - at元素目标id
    ///     - 艾特所有成员=all
    ///     - 艾特在线成员=online
    ///    当为None则默认为 all
    /// # 返回值
    ///
    /// * `AtElement` - at元素
    pub fn at(target_id: Option<String>) -> AtElement {
        AtElement {
            r#type: "at".to_string(),
            target_id: target_id.unwrap_or("all".to_string()),
            name: None,
        }
    }

    /// 创建一个表情元素
    ///
    /// # 参数
    ///
    /// * `id` - 表情元素id
    ///
    /// # 返回值
    ///
    /// * `FaceElement` - 表情元素
    pub fn face(id: Option<u64>) -> FaceElement {
        FaceElement {
            r#type: "face".to_string(),
            id,
            is_big: Some(false),
        }
    }

    /// 创建一个文件元素
    ///
    /// # 参数
    ///
    /// * `file`
    ///  - 文件网络url
    ///  - 文件绝对路径
    ///  - base64
    ///
    /// # 返回值
    ///
    /// * `FileElement` - 文件元素
    pub fn file(file: String) -> FileElement {
        FileElement {
            r#type: "file".to_string(),
            file,
        }
    }

    /// 创建一个图片元素
    ///
    /// # 参数
    ///
    /// * `url` - 图片网络url
    ///
    /// # 返回值
    ///
    /// * `ImageElement` - 图片元素
    pub fn image(file: String) -> ImageElement {
        ImageElement {
            r#type: "image".to_string(),
            file,
        }
    }

    /// 创建一个json元素
    ///
    /// # 参数
    ///
    /// * `data` - Json数据，未序列化
    ///
    /// # 返回值
    ///
    /// * `JsonElement` - json元素
    pub fn json(data: String) -> JsonElement {
        JsonElement {
            r#type: "json".to_string(),
            data,
        }
    }

    /// 创建一个音乐元素
    ///
    ///
    /// 默认音乐平台为QQ
    ///
    /// # 参数
    ///
    /// * `music_id` - 音乐元素id，如果需要自定义音乐平台，此参数请传入url再调用custom
    ///
    /// # 返回值
    ///
    /// * `MusicElement` - 音乐元素
    pub fn music(music_id: String) -> MusicElement {
        let data = MusicData::Standard {
            platform: String::new(),
            id: music_id,
        };
        MusicElement {
            r#type: "music".to_string(),
            data,
        }
        .qq()
    }

    /// 创建一个语音元素
    ///
    /// # 参数
    ///
    /// * `file`
    ///  - 语音网络url
    ///  - 语音绝对路径
    ///  - base64
    ///
    /// # 返回值
    ///
    /// * `RecordElement` - 语音元素
    pub fn record(file: String) -> RecordElement {
        RecordElement {
            r#type: "record".to_string(),
            file,
        }
    }

    /// 创建一个回复元素
    ///
    /// # 参数
    ///
    /// * `message_id` - 回复元素id
    ///
    /// # 返回值
    ///
    /// * `ReplyElement` - 回复元素
    pub fn reply(message_id: String) -> ReplyElement {
        ReplyElement {
            r#type: "reply".to_string(),
            message_id,
        }
    }

    /// 创建一个文本元素
    ///
    /// # 参数
    ///
    /// * `text` - 文本元素内容
    ///
    /// # 返回值
    ///
    /// * `TextElement` - 文本元素
    pub fn text(text: String) -> TextElement {
        TextElement {
            r#type: "text".to_string(),
            text,
        }
    }

    /// 创建一个视频元素
    ///
    /// # 参数
    ///
    /// * `file` - 视频元素id
    ///  - 视频网络url
    ///  - 视频绝对路径
    ///  - base64
    ///
    /// # 返回值
    ///
    /// * `VideoElement` - 视频元素
    pub fn video(file: String) -> VideoElement {
        VideoElement {
            r#type: "video".to_string(),
            file,
        }
    }

    /// 创建一个xml元素
    ///
    /// # 参数
    ///
    /// * `data` - Xml数据，未序列化
    ///
    /// # 返回值
    ///
    /// * `XmlElement` - xml元素
    pub fn xml(data: String) -> XmlElement {
        XmlElement {
            r#type: "xml".to_string(),
            data,
        }
    }
}

impl From<Segment> for Vec<Segment> {
    fn from(segment: Segment) -> Vec<Segment> {
        vec![segment]
    }
}

pub type SendMessage = Vec<Segment>;
