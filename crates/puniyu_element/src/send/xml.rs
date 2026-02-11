use serde::{Deserialize, Serialize};
use crate::{ElementType, RawMessage};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XmlElement<'x> {
	/// Xml数据，未序列化
	pub data: &'x str,
}

impl<'x> XmlElement<'x> {
	pub fn new(data: &'x str) -> Self {
		Self { data }
	}
}

impl<'x> RawMessage for XmlElement<'x> {
	fn r#type(&self) -> ElementType {
		ElementType::Xml
	}

	fn raw(&self) -> String {
		self.data.to_string()
	}
}