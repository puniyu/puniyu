use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonElement {
    /// json元素类型
    #[serde(rename = "type")]
    pub r#type: &'static str,
    /// Json数据，未序列化
    pub data: String,
}

impl JsonElement {
    /// 创建一个json元素
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
