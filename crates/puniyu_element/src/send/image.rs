use bon::Builder;
use serde::{Deserialize, Serialize};

use puniyu_core::element::Element;
use smol_str::SmolStr;

use crate::{ElementType, File};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Builder)]
pub struct ImageElement {
	/// 图片元素
	#[builder(into)]
	pub file: File,
	/// 图片文件名
	#[builder(into)]
	pub file_name: SmolStr,
	/// 图片外显
	#[builder(into)]
	pub summary: Option<SmolStr>,
}

impl ImageElement {
	pub fn new(
		file: impl Into<File>,
		file_name: impl Into<SmolStr>,
		summary: Option<impl Into<SmolStr>>,
	) -> Self {
		Self { file: file.into(), file_name: file_name.into(), summary: summary.map(Into::into) }
	}
}

impl Element for ImageElement {
	type ElementType = ElementType;
	fn r#type(&self) -> Self::ElementType {
		Self::ElementType::Image
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_new_with_summary() {
		let e = ImageElement::new(File::Bytes(bytes::Bytes::from_static(b"png")), "img.png", Some("hello"));
		assert_eq!(e.file_name, "img.png");
		assert_eq!(e.summary.as_ref().map(|s| s.as_str()), Some("hello"));
	}

	#[test]
	fn test_new_none_summary() {
		let e = ImageElement::new(File::Bytes(bytes::Bytes::from_static(b"png")), "img.png", None::<&str>);
		assert_eq!(e.file_name, "img.png");
		assert_eq!(e.summary, None);
	}

	#[test]
	fn test_type() {
		let e = ImageElement::new(File::Bytes(bytes::Bytes::from_static(b"png")), "i.png", None::<&str>);
		assert_eq!(e.r#type(), ElementType::Image);
	}
}
