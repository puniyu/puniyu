use serde::{Deserialize, Serialize};
use crate::{ElementType, RawMessage};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonElement<'j> {
    /// Json数据，未序列化
    pub data: &'j str,
}

impl<'j> RawMessage for JsonElement<'j> {
    fn r#type(&self) -> ElementType {
        ElementType::Json
    }

    fn raw(&self) -> String {
        self.data.to_string()
    }
}

impl<'t> From<JsonElement<'t>> for String {
    fn from(JsonElement { data }: JsonElement<'t>) -> Self {
        data.to_string()
    }
}
impl<'t> From<JsonElement<'t>> for &'t str {
    fn from(JsonElement { data }: JsonElement<'t>) -> Self {
        data
    }
}

impl<'t> From<&'t str> for JsonElement<'t> {
    fn from(text: &'t str) -> Self {
        Self { data: text }
    }
}
