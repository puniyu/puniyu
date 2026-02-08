use serde::{Deserialize, Serialize};
use super::RawMessage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyElement<'r> {
    /// 回复元素id
    pub message_id: &'r str,
}

impl<'r> RawMessage for ReplyElement<'r> {
    fn raw(&self) -> String {
        format!("[reply:{}]", self.message_id)
    }
}

impl<'r> From<ReplyElement<'r>> for String {
    fn from(elem: ReplyElement) -> Self {
        elem.message_id.to_string()
    }
}

impl<'r> From<&'r str> for ReplyElement<'r> {
    fn from(text: &'r str) -> Self {
        Self { message_id: text }
    }
}
