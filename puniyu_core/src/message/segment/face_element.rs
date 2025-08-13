use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FaceElement {
    /// 元素类型
    #[serde(rename = "type")]
    pub r#type: String,
    /// 表情元素id
    pub id: u64,
    /// 是否为大表情
    pub is_big: Option<bool>,
}

impl FaceElement {
    /// 创建一个表情元素
    ///
    /// # 参数
    ///
    /// * `id` - 表情元素id
    /// * `is_big` - 表情元素是否为大表情
    ///
    /// # 返回值
    ///
    /// * `FaceElement` - 表情元素
    pub fn new(id: u64, is_big: Option<bool>) -> Self {
        Self {
            r#type: "face".to_string(),
            id,
            is_big,
        }
    }

    /// 创建一个普通表情元素
    ///
    /// # 参数
    ///
    /// * `id` - 表情元素id
    ///
    /// # 返回值
    ///
    /// * `FaceElement` - 普通表情元素
    pub fn normal(id: u64) -> Self {
        Self::new(id, None)
    }
    /// 创建一个大表情元素
    ///
    /// # 参数
    ///
    /// * `id` - 表情元素id
    ///
    /// # 返回值
    ///
    /// * `FaceElement` - 大表情元素
    pub fn big(id: u64) -> Self {
        Self::new(id, Some(true))
    }
}
