use super::{impl_element_from, impl_vec_element_from};
use puniyu_element::receive as puniyu_element;

include!(concat!(env!("OUT_DIR"), "/puniyu.element.receive.rs"));

impl_element_from!(map<'a>, &'a TextElement => puniyu_element::TextElement<'a>, value, {
	text: value.text.as_str()
});
impl_element_from!(map<'a>, &'a AtElement => puniyu_element::AtElement<'a>, value, {
	target_id: value.target_id.as_str()
});
impl_element_from!(map<'a>, &'a ReplyElement => puniyu_element::ReplyElement<'a>, value, {
	message_id: value.message_id.as_str()
});
impl_element_from!(map<'a>, &'a FaceElement => puniyu_element::FaceElement, value, {
	id: value.id
});
impl_element_from!(map<'a>, &'a ImageElement => puniyu_element::ImageElement<'a>, value, {
	file: value.file.clone(),
	file_name: value.file_name.as_str(),
	summary: value.summary.as_str(),
	width: value.width,
	height: value.height
});
impl_element_from!(map<'a>, &'a FileElement => puniyu_element::FileElement<'a>, value, {
	file: value.file.clone(),
	file_size: value.file_size,
	file_name: value.file_name.as_str()
});
impl_element_from!(map<'a>, &'a VideoElement => puniyu_element::VideoElement<'a>, value, {
	file: value.file.clone(),
	file_name: value.file_name.as_str()
});
impl_element_from!(map<'a>, &'a RecordElement => puniyu_element::RecordElement<'a>, value, {
	file: value.file.clone(),
	file_name: value.file_name.as_str()
});
impl_element_from!(map<'a>, &'a JsonElement => puniyu_element::JsonElement<'a>, value, {
	data: value.data.as_str()
});
impl_element_from!(map<'a>, &'a XmlElement => puniyu_element::XmlElement<'a>, value, {
	data: value.data.as_str()
});

impl_element_from!(map, puniyu_element::TextElement<'_> => TextElement, value, {
	text: value.text.to_string()
});
impl_element_from!(map, puniyu_element::AtElement<'_> => AtElement, value, {
	target_id: value.target_id.to_string()
});
impl_element_from!(map, puniyu_element::ReplyElement<'_> => ReplyElement, value, {
	message_id: value.message_id.to_string()
});
impl_element_from!(map, puniyu_element::FaceElement => FaceElement, value, {
	id: value.id
});
impl_element_from!(map, puniyu_element::ImageElement<'_> => ImageElement, value, {
	file: value.file,
	file_name: value.file_name.to_string(),
	summary: value.summary.to_string(),
	width: value.width,
	height: value.height
});
impl_element_from!(map, puniyu_element::FileElement<'_> => FileElement, value, {
	file: value.file,
	file_size: value.file_size,
	file_name: value.file_name.to_string()
});
impl_element_from!(map, puniyu_element::VideoElement<'_> => VideoElement, value, {
	file: value.file,
	file_name: value.file_name.to_string()
});
impl_element_from!(map, puniyu_element::RecordElement<'_> => RecordElement, value, {
	file: value.file,
	file_name: value.file_name.to_string()
});
impl_element_from!(map, puniyu_element::JsonElement<'_> => JsonElement, value, {
	data: value.data.to_string()
});
impl_element_from!(map, puniyu_element::XmlElement<'_> => XmlElement, value, {
	data: value.data.to_string()
});

impl_element_from!(
	enum, puniyu_element::Elements<'_> => element::Element, {
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
	oneof<'a>, &'a Element => puniyu_element::Elements<'a>, value, {
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
	none = "puniyu_protobuf::element::receive::Element.element cannot be None"
);

impl_element_from!(map, puniyu_element::Elements<'_> => Element, value, {
	element: Some(value.into())
});

impl_vec_element_from!(
	map<'a>,
	&'a Elements => Vec<puniyu_element::Elements<'a>>,
	elements,
	elements.element.iter().map(Into::into)
);
