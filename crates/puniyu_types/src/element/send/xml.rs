use serde::{Deserialize, Serialize};

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

impl<'x> From<&'x str> for XmlElement<'x> {
	fn from(data: &'x str) -> Self {
		Self { data }
	}
}
