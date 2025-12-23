use crate::element::receive::elements::Element;
use puniyu_types::element::receive as element;

include!(concat!(env!("OUT_DIR"), "/puniyu.element.receive.rs"));

macro_rules! impl_from_element {
    ($proto_type:ty, $internal_type:ty { $($field:ident),+ }) => {
        impl From<$proto_type> for $internal_type {
            fn from(value: $proto_type) -> Self {
                Self {
                    $($field: value.$field),+
                }
            }
        }

        impl From<$internal_type> for $proto_type {
            fn from(value: $internal_type) -> Self {
                Self {
                    $($field: value.$field),+
                }
            }
        }
    };
}

impl_from_element!(TextElement, element::TextElement { text });
impl_from_element!(AtElement, element::AtElement { target_id });
impl_from_element!(ReplyElement, element::ReplyElement { message_id });
impl_from_element!(FaceElement, element::FaceElement { id });
impl_from_element!(ImageElement, element::ImageElement { file, summary, width, height });
impl_from_element!(FileElement, element::FileElement { file, file_id, file_size, file_name });
impl_from_element!(VideoElement, element::VideoElement { file, file_name });
impl_from_element!(RecordElement, element::RecordElement { file });
impl_from_element!(JsonElement, element::JsonElement { data });
impl_from_element!(XmlElement, element::XmlElement { data });

impl From<element::Elements> for Element {
	fn from(value: element::Elements) -> Self {
		match value {
			element::Elements::Text(text) => Self::TextElement(text.into()),
			element::Elements::At(at) => Self::AtElement(at.into()),
			element::Elements::Reply(reply) => Self::ReplyElement(reply.into()),
			element::Elements::Face(face) => Self::FaceElement(face.into()),
			element::Elements::Image(image) => Self::ImageElement(image.into()),
			element::Elements::File(file) => Self::FileElement(file.into()),
			element::Elements::Video(video) => Self::VideoElement(video.into()),
			element::Elements::Record(record) => Self::RecordElement(record.into()),
			element::Elements::Json(json) => Self::JsonElement(json.into()),
			element::Elements::Xml(xml) => Self::XmlElement(xml.into()),
		}
	}
}

impl From<Element> for element::Elements {
	fn from(value: Element) -> Self {
		match value {
			Element::TextElement(text) => Self::Text(text.into()),
			Element::AtElement(at) => Self::At(at.into()),
			Element::ReplyElement(reply) => Self::Reply(reply.into()),
			Element::FaceElement(face) => Self::Face(face.into()),
			Element::ImageElement(image) => Self::Image(image.into()),
			Element::FileElement(file) => Self::File(file.into()),
			Element::VideoElement(video) => Self::Video(video.into()),
			Element::RecordElement(record) => Self::Record(record.into()),
			Element::JsonElement(json) => Self::Json(json.into()),
			Element::XmlElement(xml) => Self::Xml(xml.into()),
		}
	}
}

impl From<element::Elements> for Elements {
	fn from(value: element::Elements) -> Self {
		Self { element: Some(Element::from(value)) }
	}
}

impl From<Elements> for element::Elements {
	fn from(value: Elements) -> Self {
		value.element.unwrap().into()
	}
}
