use serde::{Deserialize, Serialize};

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

impl<'j> From<&'j str> for JsonElement<'j> {
	fn from(data: &'j str) -> Self {
		Self { data }
	}
}
