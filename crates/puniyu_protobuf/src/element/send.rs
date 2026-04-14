use super::{impl_element_from, impl_vec_element_from};
use puniyu_element::send as puniyu_element;

include!(concat!(env!("OUT_DIR"), "/puniyu.element.send.rs"));

impl_element_from!(bi, TextElement, puniyu_element::TextElement, { text });
impl_element_from!(bi, AtElement, puniyu_element::AtElement, { target_id });
impl_element_from!(bi, ReplyElement, puniyu_element::ReplyElement, { message_id });
impl_element_from!(bi, FaceElement, puniyu_element::FaceElement, { id });
impl_element_from!(bi, ImageElement, puniyu_element::ImageElement, { file, file_name, summary });
impl_element_from!(bi, FileElement, puniyu_element::FileElement, { file, file_name });
impl_element_from!(bi, VideoElement, puniyu_element::VideoElement, { file, file_name });
impl_element_from!(bi, RecordElement, puniyu_element::RecordElement, { file, file_name });
impl_element_from!(bi, JsonElement, puniyu_element::JsonElement, { data });
impl_element_from!(bi, XmlElement, puniyu_element::XmlElement, { data });

impl_element_from!(
	enum, puniyu_element::Elements, element::Element, {
		puniyu_element::Elements::Text => element::Element::TextElement,
		puniyu_element::Elements::At => element::Element::AtElement,
		puniyu_element::Elements::Reply => element::Element::ReplyElement,
		puniyu_element::Elements::Face => element::Element::FaceElement,
		puniyu_element::Elements::Image => element::Element::ImageElement,
		puniyu_element::Elements::File => element::Element::FileElement,
		puniyu_element::Elements::Video => element::Element::VideoElement,
		puniyu_element::Elements::Record => element::Element::RecordElement,
		puniyu_element::Elements::Json => element::Element::JsonElement,
		puniyu_element::Elements::Xml => element::Element::XmlElement,
	}
);

impl_element_from!(
	oneof, Element, puniyu_element::Elements, {
		element::Element::TextElement => puniyu_element::Elements::Text,
		element::Element::AtElement => puniyu_element::Elements::At,
		element::Element::ReplyElement => puniyu_element::Elements::Reply,
		element::Element::FaceElement => puniyu_element::Elements::Face,
		element::Element::ImageElement => puniyu_element::Elements::Image,
		element::Element::FileElement => puniyu_element::Elements::File,
		element::Element::VideoElement => puniyu_element::Elements::Video,
		element::Element::RecordElement => puniyu_element::Elements::Record,
		element::Element::JsonElement => puniyu_element::Elements::Json,
		element::Element::XmlElement => puniyu_element::Elements::Xml,
	},
	none = "puniyu_protobuf::element::send::Element.element cannot be None"
);

impl_element_from!(map, puniyu_element::Elements, Element, value, {
	element: Some(value.into())
});

impl_vec_element_from!(bi, puniyu_element::Elements, Elements);
