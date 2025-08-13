use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
// TODO: 接受视频元素能获取更多信息
pub struct VideoElement {
    /// 元素类型
    #[serde(rename = "type")]
    r#type: String,
    /// 视频元素id
    ///  - 视频网络url
    ///  - 视频绝对路径
    ///  - base64
    file: String,
}

impl VideoElement {
    /// 创建一个视频元素
    ///
    /// # 参数
    ///
    /// * `file`
    ///  - 视频网络url
    ///  - 视频绝对路径
    ///  - base64
    ///
    /// # 返回值
    ///
    /// * `VideoElement` - 视频元素
    pub fn new(file: String) -> Self {
        Self {
            r#type: "video".to_string(),
            file,
        }
    }
}
