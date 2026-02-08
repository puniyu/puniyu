use serde::{Deserialize, Serialize};
use super::RawMessage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonElement<'j> {
    /// Json数据，未序列化
    pub data: &'j str,
}

impl<'j> RawMessage for JsonElement<'j> {
    fn raw(&self) -> String {
        format!("[json:{}]", self.data)
    }
}