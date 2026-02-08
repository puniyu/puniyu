use serde::{Deserialize, Serialize};
use super::RawMessage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceElement {
    /// 表情元素id
    pub id: u64,
}

impl RawMessage for FaceElement {
    fn raw(&self) -> String {
        format!("[face:{}]", self.id)
    }
}

impl From<FaceElement> for String {
    fn from(elem: FaceElement) -> Self {
        elem.id.to_string()
    }
}

impl<'t> From<&'t str> for FaceElement {
    fn from(text: &'t str) -> Self {
        Self { id: text.parse().unwrap_or_default() }
    }
}
