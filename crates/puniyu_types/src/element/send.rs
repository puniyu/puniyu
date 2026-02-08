macro_rules! codegen_reexport {
    ($($module:ident => $type:ident),*) => {
        $(
            mod $module;
			#[doc(inline)]
            pub use $module::$type;
        )*
    };
}

codegen_reexport! {
	text => TextElement,
	at => AtElement,
	reply => ReplyElement,
	face => FaceElement,
	image => ImageElement,
	file => FileElement,
	video => VideoElement,
	record => RecordElement,
	json => JsonElement,
	xml => XmlElement
}

use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[serde(rename_all = "lowercase", tag = "type", content = "field0", bound(deserialize = "'de: 'e"))]
pub enum Elements<'e> {
	Text(TextElement<'e>),
	At(AtElement<'e>),
	Reply(ReplyElement<'e>),
	Face(FaceElement),
	Image(ImageElement<'e>),
	File(FileElement<'e>),
	Video(VideoElement<'e>),
	Record(RecordElement<'e>),
	Json(JsonElement<'e>),
	Xml(XmlElement<'e>),
}

impl<'e> Elements<'e> {
	pub fn as_text(&self) -> Option<&str> {
		match self {
			Elements::Text(text_element) => Some(text_element.text),
			_ => None,
		}
	}

	pub fn as_at(&self) -> Option<&AtElement> {
		match self {
			Elements::At(element) => Some(element),
			_ => None,
		}
	}

	pub fn as_reply(&self) -> Option<&ReplyElement> {
		match self {
			Elements::Reply(element) => Some(element),
			_ => None,
		}
	}

	pub fn as_face(&self) -> Option<&FaceElement> {
		match self {
			Elements::Face(element) => Some(element),
			_ => None,
		}
	}

	pub fn as_image(&self) -> Option<&ImageElement> {
		match self {
			Elements::Image(element) => Some(element),
			_ => None,
		}
	}

	pub fn as_file(&self) -> Option<&FileElement> {
		match self {
			Elements::File(element) => Some(element),
			_ => None,
		}
	}

	pub fn as_video(&self) -> Option<&VideoElement> {
		match self {
			Elements::Video(element) => Some(element),
			_ => None,
		}
	}

	pub fn as_record(&self) -> Option<&RecordElement> {
		match self {
			Elements::Record(element) => Some(element),
			_ => None,
		}
	}

	pub fn as_json(&self) -> Option<&JsonElement> {
		match self {
			Elements::Json(element) => Some(element),
			_ => None,
		}
	}

	pub fn as_xml(&self) -> Option<&XmlElement> {
		match self {
			Elements::Xml(element) => Some(element),
			_ => None,
		}
	}
}
