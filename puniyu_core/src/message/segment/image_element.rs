use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageElement {
    /// 元素类型
    #[serde(rename = "type")]
    pub r#type: &'static str,
    /// 图片元素id
    ///  - 图片网络url
    ///  - 图片绝对路径
    ///  - base64
    pub file: Result<String, PathBuf>,
}

impl ImageElement {
    /// 创建一个图片元素
    ///
    /// # 参数
    ///
    /// * `file`
    ///  - 图片网络url
    ///  - 图片绝对路径
    ///  - base64
    ///
    /// # 返回值
    ///
    /// * `ImageElement` - 图片元素
    pub fn new(file: Result<String, PathBuf>) -> Self {
        Self {
            r#type: "image",
            file,
        }
    }
}
