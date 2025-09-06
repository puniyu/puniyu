use crate::message::music::{CustomMusicOptions, MusicData, MusicPlatform};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AtElement {
    /// at元素类型
    #[serde(rename = "type")]
    pub r#type: String,
    /// at元素目标id
    pub target_id: String,
    /// at元素目标名称
    pub name: Option<String>,
}

impl AtElement {
    /// 为at元素设置目标名称
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
    /// 为at元素设置目标为所有成员
    pub fn all(mut self) -> Self {
        self.target_id = "all".to_string();
        self.name = None;
        self
    }
    /// 为at元素设置目标为在线成员
    pub fn online(mut self) -> Self {
        self.target_id = "online".to_string();
        self.name = None;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FaceElement {
    /// 元素类型
    #[serde(rename = "type")]
    pub r#type: String,
    /// 表情元素id
    pub id: Option<u64>,
    /// 是否为大表情, 默认false
    pub is_big: Option<bool>,
}

impl FaceElement {
    /// 设置为大表情
    pub fn big(mut self) -> Self {
        self.is_big = Some(true);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileElement {
    /// 元素类型
    #[serde(rename = "type")]
    pub r#type: String,
    /// 文件元素id
    ///  - 文件网络url
    ///  - 文件绝对路径
    ///  - base64
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
// TODO: 接受图片元素能获取更多信息
pub struct ImageElement {
    /// 元素类型
    #[serde(rename = "type")]
    pub r#type: String,
    /// 图片元素id
    ///  - 图片网络url
    ///  - 图片绝对路径
    ///  - base64
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonElement {
    /// json元素类型
    #[serde(rename = "type")]
    pub r#type: String,
    /// Json数据，未序列化
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicElement {
    /// 元素类型
    #[serde(rename = "type")]
    pub r#type: String,
    /// 音乐数据
    /// 自动展开
    #[serde(flatten)]
    pub data: MusicData,
}

impl MusicElement {
    /// 创建QQ音乐元素，默认
    pub fn qq(mut self) -> Self {
        if let MusicData::Standard { id, .. } = self.data {
            self.data = MusicData::Standard {
                platform: MusicPlatform::QQ.to_platform_string(),
                id,
            };
        }
        self
    }

    /// 创建网易云音乐元素
    pub fn net_ease(mut self) -> Self {
        if let MusicData::Standard { id, .. } = self.data {
            self.data = MusicData::Standard {
                platform: MusicPlatform::NetEase.to_platform_string(),
                id,
            };
        }
        self
    }

    /// 创建咪咕音乐元素
    pub fn migu(mut self) -> Self {
        if let MusicData::Standard { id, .. } = self.data {
            self.data = MusicData::Standard {
                platform: MusicPlatform::MiGu.to_platform_string(),
                id,
            };
        }
        self
    }

    /// 创建酷狗音乐元素
    pub fn kugou(mut self) -> Self {
        if let MusicData::Standard { id, .. } = self.data {
            self.data = MusicData::Standard {
                platform: MusicPlatform::KuGou.to_platform_string(),
                id,
            };
        }
        self
    }

    /// 创建酷我音乐元素
    pub fn kuwo(mut self) -> Self {
        if let MusicData::Standard { id, .. } = self.data {
            self.data = MusicData::Standard {
                platform: MusicPlatform::Kuwo.to_platform_string(),
                id,
            };
        }
        self
    }

    /// 创建自定义音乐元素
    pub fn custom(mut self, options: CustomMusicOptions) -> Self {
        // 只会拿到Standard的id并作为url
        let music_id = match self.data {
            MusicData::Standard { id, .. } => id,
            MusicData::Custom { url, .. } => url,
        };
        self.data = MusicData::Custom {
            platform: MusicPlatform::Custom.to_platform_string(),
            url: music_id,
            audio: options.audio,
            title: options.title,
            author: options.author,
            pic: options.pic,
        };
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordElement {
    /// 元素类型
    #[serde(rename = "type")]
    pub r#type: String,
    /// 语言元素
    ///  - 语音网络url
    ///  - 语音绝对路径
    ///  - base64
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplyElement {
    /// 元素类型
    #[serde(rename = "type")]
    pub r#type: String,
    /// 回复元素id
    #[serde(rename = "messageId")]
    pub message_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextElement {
    /// 文本元素类型
    #[serde(rename = "type")]
    pub r#type: String,
    /// 文本元素内容
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoElement {
    /// 元素类型
    #[serde(rename = "type")]
    pub r#type: String,
    /// 视频元素id
    ///  - 视频网络url
    ///  - 视频绝对路径
    ///  - base64
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XmlElement {
    /// xml元素类型
    #[serde(rename = "type")]
    pub r#type: String,
    /// Xml数据，未序列化
    pub data: String,
}

pub enum Element {
    At(AtElement),
    Image(ImageElement),
    Json(JsonElement),
    Music(MusicElement),
    Record(RecordElement),
    Reply(ReplyElement),
    Text(TextElement),
    Video(VideoElement),
    Xml(XmlElement),
}
