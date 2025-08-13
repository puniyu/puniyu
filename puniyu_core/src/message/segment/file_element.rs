use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileElement {
    /// 元素类型
    #[serde(rename = "type")]
    pub r#type: &'static str,
    /// 文件元素id
    ///  - 文件网络url
    ///  - 文件绝对路径
    ///  - base64
    pub file: Result<String, PathBuf>,
}

impl FileElement {
    /// 创建一个文件元素
    ///
    /// # 参数
    ///
    /// * `file`
    ///  - 文件网络url
    ///  - 文件绝对路径
    ///  - base64
    ///
    /// # 返回值
    ///
    /// * `FileElement` - 文件元素
    pub fn new(file: Result<String, PathBuf>) -> Self {
        Self {
            r#type: "file",
            file,
        }
    }
}
