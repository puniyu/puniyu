use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
// TODO: 接受文件元素能获取更多信息
pub struct FileElement {
    /// 元素类型
    #[serde(rename = "type")]
    pub r#type: String,
    /// 文件元素id
    ///  - 文件网络url
    ///  - 文件绝对路径
    ///  - base64
    pub file: String,
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
    pub fn new(file: String) -> Self {
        Self {
            r#type: "file".to_string(),
            file,
        }
    }
}
