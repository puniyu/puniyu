use bytes::Bytes;
use serde::{Deserialize, Serialize};
use crate::{ElementType, RawMessage};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileElement<'f> {
    /// 文件元素
    pub file: Bytes,
    /// 文件大小, 单位字节
    pub file_size: u64,
    /// 文件名称
    pub file_name: &'f str,
}

impl<'f> RawMessage for FileElement<'f> {
    fn r#type(&self) -> ElementType {
        ElementType::File
    }

    fn raw(&self) -> String {
        self.file_name.to_string()
    }
}
