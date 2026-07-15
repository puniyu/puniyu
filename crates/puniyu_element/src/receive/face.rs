use std::str::FromStr;

use serde::{Deserialize, Serialize};
use crate::{Element, ElementType};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FaceElement {
	/// 表情元素id
	pub id: u64,
}

impl FromStr for FaceElement {
	type Err = <u64 as FromStr>::Err;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Self { id: s.parse()? })
	}
}

impl From<FaceElement> for String {
	fn from(elem: FaceElement) -> Self {
		elem.id.to_string()
	}
}


impl Element for FaceElement {
	type ElementType = ElementType;

    fn r#type(&self) -> Self::ElementType {
        Self::ElementType::Face
    }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_new_and_type() {
		let e = FaceElement { id: 1 };
		assert_eq!(e.id, 1);
		assert_eq!(e.r#type(), ElementType::Face);
	}


	#[test]
	fn test_serde_roundtrip() {
		let e = FaceElement { id: 100 };
		let json = serde_json::to_string(&e).expect("serialize");
		let restored: FaceElement = serde_json::from_str(&json).expect("deserialize");
		assert_eq!(restored.id, 100);
	}
}
