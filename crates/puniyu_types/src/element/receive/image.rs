use bytes::Bytes;
use serde::{Deserialize, Serialize};
use super::RawMessage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageElement<'m> {
    /// 图片元素
    pub file: Bytes,
    /// 图片外显
    pub summary: &'m str,
    /// 图片宽度
    pub width: u32,
    /// 图片高度
    pub height: u32,
}

impl<'m> RawMessage for ImageElement<'m> {
    fn raw(&self) -> String {
        format!("[image:{}]", self.summary)
    }
}
