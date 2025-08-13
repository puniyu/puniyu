use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XmlElement {
    /// json元素类型
    #[serde(rename = "type")]
    pub r#type: &'static str,
    /// Xml数据，未序列化
    pub data: String,
}

impl XmlElement {
    /// 创建一个xml元素
    ///
    /// # 参数
    ///
    /// * `data` - Json数据，未序列化
    ///
    /// # 返回值
    ///
    /// * `JsonElement` - json元素
    pub fn new(data: String) -> Self {
        Self {
            r#type: "json",
            data,
        }
    }
}
