use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoElement {
    /// 元素类型
    #[serde(rename = "type")]
    r#type: &'static str,
    /// 视频元素id
    ///  - 视频网络url
    ///  - 视频绝对路径
    ///  - base64
    file: Result<String, PathBuf>,
}
