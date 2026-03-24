use bytes::Bytes;
use serde::{Deserialize, Serialize};
use crate::{Element, ElementType, RawMessage};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VideoElement<'v> {
    /// 视频元素
    pub file: Bytes,
    /// 视频文件名
    pub file_name: &'v str,
}

impl<'v> Element for VideoElement<'v> {
    fn r#type(&self) -> ElementType {
        ElementType::Video
    }
}

impl<'v> RawMessage for VideoElement<'v> {
    fn raw(&self) -> String {
        self.file_name.to_string()
    }
}