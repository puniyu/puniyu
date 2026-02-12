use bytes::Bytes;
use serde::{Deserialize, Serialize};
use crate::{ElementType, RawMessage};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VideoElement<'v> {
    /// 视频元素
    pub file: Bytes,
    /// 视频文件名
    pub file_name: &'v str,
}

impl<'v> RawMessage for VideoElement<'v> {
    fn r#type(&self) -> ElementType {
        ElementType::Video
    }

    fn raw(&self) -> String {
        self.file_name.to_string()
    }
}