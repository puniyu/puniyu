use serde::{Deserialize, Serialize};
use crate::{ElementType, RawMessage};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonElement<'j> {
	/// Json数据，未序列化
	pub data: &'j str,
}

impl<'j> JsonElement<'j> {
	pub fn new(data: &'j str) -> Self {
		Self { data }
	}
}

impl<'j> RawMessage for JsonElement<'j> {
	fn r#type(&self) -> ElementType {
		ElementType::Json
	}

	fn raw(&self) -> String {
		self.data.to_string()
	}
}