use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextElement {
    /// 文本元素类型
    #[serde(rename = "type")]
    r#type: &'static str,
    /// 文本元素内容
    text: String,
}
impl TextElement {
    /// 创建一个文本元素
    ///
    /// # 参数
    ///
    /// * `text` - 文本元素内容
    ///
    /// # 返回值
    ///
    /// * `TextElement` - 文本元素
    pub fn new(text: String) -> Self {
        Self {
            r#type: "text",
            text,
        }
    }
}
