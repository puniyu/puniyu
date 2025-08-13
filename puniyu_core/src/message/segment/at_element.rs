use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AtElement {
    /// at元素类型
    #[serde(rename = "type")]
    pub r#type: String,
    /// at元素目标id
    pub target_id: String,
    /// at元素目标名称
    pub name: Option<String>,
}

impl AtElement {
    /// 创建一个at元素
    ///
    /// # 参数
    ///
    /// * `target_id`
    ///     - at元素目标id
    ///     - 艾特所有成员=all
    ///     - 艾特在线成员=online
    /// * `name` - at元素目标名称
    ///
    /// # 返回值
    ///
    /// * `AtElement` - at元素
    pub fn new(target_id: String, name: Option<String>) -> Self {
        Self {
            r#type: "at".to_string(),
            target_id,
            name,
        }
    }

    /// 创建一个at所有成员元素
    ///
    /// # 返回值
    ///
    /// * `AtElement` - at所有成员元素
    pub fn all() -> Self {
        Self::new("all".to_string(), None)
    }

    /// 创建一个at在线成员元素
    ///
    /// # 返回值
    ///
    /// * `AtElement` - at在线成员元素
    pub fn online() -> Self {
        Self::new("online".to_string(), None)
    }
}
