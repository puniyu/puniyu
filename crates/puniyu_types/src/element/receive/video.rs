use bytes::Bytes;
use serde::{Deserialize, Serialize};
use super::RawMessage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoElement<'v> {
    /// 视频元素
    pub file: Bytes,
    /// 视频文件名
    pub file_name: &'v str,
}

impl<'v> RawMessage for VideoElement<'v> {
    fn raw(&self) -> String {
        format!("[video:{}]", self.file_name)
    }
}