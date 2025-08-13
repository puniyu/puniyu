use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyElement {
    /// 元素类型
    #[serde(rename = "type")]
    r#type: &'static str,
    /// 回复元素id
    #[serde(rename = "messageId")]
    message_id: u64,
}
impl ReplyElement {
    /// 创建一个回复元素
    ///
    /// # 参数
    ///
    /// * `message_id` - 回复元素id
    ///
    /// # 返回值
    ///
    /// * `ReplyElement` - 回复元素
    pub fn new(message_id: u64) -> Self {
        Self {
            r#type: "reply",
            message_id,
        }
    }
}
