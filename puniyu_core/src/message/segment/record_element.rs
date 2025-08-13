use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordElement {
    /// 元素类型
    #[serde(rename = "type")]
    r#type: String,
    /// 语言元素
    ///  - 语音网络url
    ///  - 语音绝对路径
    ///  - base64
    file: Result<String, PathBuf>,
}

impl RecordElement {
    /// 创建一个语音元素
    ///
    /// # 参数
    ///
    /// * `file`
    ///  - 语音网络url
    ///  - 语音绝对路径
    ///  - base64
    ///
    /// # 返回值
    ///
    /// * `RecordElement` - 语音元素
    pub fn new(file: Result<String, PathBuf>) -> Self {
        Self {
            r#type: "record".to_string(),
            file,
        }
    }
}
