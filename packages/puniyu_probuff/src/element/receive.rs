pub(crate) use puniyu_types::element::receive as puniyu_element;
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

impl_from_element!(TextElement, puniyu_element::TextElement { text });
impl_from_element!(AtElement, puniyu_element::AtElement { target_id });
impl_from_element!(ReplyElement, puniyu_element::ReplyElement { message_id });
impl_from_element!(FaceElement, puniyu_element::FaceElement { id });
impl_from_element!(ImageElement, puniyu_element::ImageElement { file, summary, width, height });
impl_from_element!(
	FileElement,
	puniyu_element::FileElement { file, file_id, file_size, file_name }
);
impl_from_element!(VideoElement, puniyu_element::VideoElement { file, file_name });
impl_from_element!(RecordElement, puniyu_element::RecordElement { file });
impl_from_element!(JsonElement, puniyu_element::JsonElement { data });
impl_from_element!(XmlElement, puniyu_element::XmlElement { data });

impl From<puniyu_element::Elements> for element::Element {
	fn from(value: puniyu_element::Elements) -> Self {
		match value {
			puniyu_element::Elements::Text(text) => Self::TextElement(text.into()),
			puniyu_element::Elements::At(at) => Self::AtElement(at.into()),
			puniyu_element::Elements::Reply(reply) => Self::ReplyElement(reply.into()),
			puniyu_element::Elements::Face(face) => Self::FaceElement(face.into()),
			puniyu_element::Elements::Image(image) => Self::ImageElement(image.into()),
			puniyu_element::Elements::File(file) => Self::FileElement(file.into()),
			puniyu_element::Elements::Video(video) => Self::VideoElement(video.into()),
			puniyu_element::Elements::Record(record) => Self::RecordElement(record.into()),
			puniyu_element::Elements::Json(json) => Self::JsonElement(json.into()),
			puniyu_element::Elements::Xml(xml) => Self::XmlElement(xml.into()),
		}
	}
}

impl From<element::Element> for puniyu_element::Elements {
	fn from(value: element::Element) -> Self {
		match value {
			element::Element::TextElement(text) => Self::Text(text.into()),
			element::Element::AtElement(at) => Self::At(at.into()),
			element::Element::ReplyElement(reply) => Self::Reply(reply.into()),
			element::Element::FaceElement(face) => Self::Face(face.into()),
			element::Element::ImageElement(image) => Self::Image(image.into()),
			element::Element::FileElement(file) => Self::File(file.into()),
			element::Element::VideoElement(video) => Self::Video(video.into()),
			element::Element::RecordElement(record) => Self::Record(record.into()),
			element::Element::JsonElement(json) => Self::Json(json.into()),
			element::Element::XmlElement(xml) => Self::Xml(xml.into()),
		}
	}
}

impl From<puniyu_element::Elements> for Elements {
	fn from(element: puniyu_element::Elements) -> Self {
		Self { element: vec![element.into()] }
	}
}

impl From<Elements> for puniyu_element::Elements {
	fn from(elements: Elements) -> Self {
		elements.element.into_iter().map(|element| element.into()).next().unwrap()
	}
}

impl From<puniyu_element::Elements> for Element {
	fn from(element: puniyu_element::Elements) -> Self {
		Self { element: Some(element.into()) }
	}
}

impl From<Element> for puniyu_element::Elements {
	fn from(element: Element) -> Self {
		element.element.unwrap().into()
	}
}
