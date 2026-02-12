use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

#[derive(Debug, Clone, PartialEq, EnumString, Display, IntoStaticStr, Deserialize, Serialize)]
pub enum ElementType {
    #[strum(serialize = "at")]
    /// 艾特元素
    At,
    #[strum(serialize = "reply")]
    /// 回复元素
    Reply,
    #[strum(serialize = "text")]
    /// 文本元素
    Text,
    #[strum(serialize = "image")]
    /// 图片元素
    Image,
    #[strum(serialize = "file")]
    /// 文件元素
    File,
    #[strum(serialize = "record")]
    /// 语音元素
    Record,
    #[strum(serialize = "video")]
    /// 视频元素
    Video,
    #[strum(serialize = "face")]
    /// 表情元素
    Face,
    #[strum(serialize = "json")]
    /// json元素
    Json,
    #[strum(serialize = "xml")]
    /// xml元素
    Xml,
}

/// 原始消息
pub trait RawMessage: Send + Sync {
    /// 消息类型
    fn r#type(&self) -> ElementType;
    /// 消息内容
    fn raw(&self) -> String;
}


impl PartialEq for dyn RawMessage {
    fn eq(&self, other: &Self) -> bool {
        self.r#type() == other.r#type() && self.raw() == other.raw()
    }
}
impl Eq for dyn RawMessage {}
