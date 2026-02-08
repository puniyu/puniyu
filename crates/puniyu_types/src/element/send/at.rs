use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtElement<'a> {
	/// at元素目标id
	pub target_id: &'a str,
}

impl<'a> AtElement<'a> {
	pub fn new(target_id: &'a str) -> Self {
		Self { target_id }
	}

	/// 是否为艾特全体
	pub fn is_everyone(&self) -> bool {
		matches!(self.target_id, "all")
	}
}

impl<'a> From<&'a str> for AtElement<'a> {
	fn from(target_id: &'a str) -> Self {
		Self { target_id }
	}
}

impl<'a> From<AtElement<'a>> for String {
	fn from(elem: AtElement<'a>) -> Self {
		elem.target_id.to_string()
	}
}

impl<'a> From<AtElement<'a>> for &'a str {
	fn from(elem: AtElement<'a>) -> Self {
		elem.target_id
	}
}
