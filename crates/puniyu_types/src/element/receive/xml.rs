use serde::{Deserialize, Serialize};
use super::RawMessage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XmlElement<'x> {
    /// Xml数据，未序列化
    pub data: &'x str,
}

impl<'x> RawMessage for XmlElement<'x> {
    fn raw(&self) -> String {
        format!("[xml:{}]", self.data)
    }
}