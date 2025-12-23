use puniyu_types::element::receive as element;

include!(concat!(env!("OUT_DIR"), "/puniyu.element.receive.rs"));

macro_rules! impl_receive_element {
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

impl_receive_element!(self::TextElement, element::TextElement { text });
impl_receive_element!(self::AtElement, element::AtElement { target_id });
impl_receive_element!(self::ReplyElement, element::ReplyElement { message_id });
impl_receive_element!(self::FaceElement, element::FaceElement { id });
impl_receive_element!(self::ImageElement, element::ImageElement { file, summary, width, height });
impl_receive_element!(
	self::FileElement,
	element::FileElement { file, file_id, file_size, file_name }
);
impl_receive_element!(self::VideoElement, element::VideoElement { file, file_name });
impl_receive_element!(self::RecordElement, element::RecordElement { file });

impl From<self::JsonElement> for element::JsonElement {
	fn from(value: self::JsonElement) -> Self {
		Self { data: value.json }
	}
}

impl From<element::JsonElement> for self::JsonElement {
	fn from(value: element::JsonElement) -> Self {
		Self { json: value.data }
	}
}

impl From<self::XmlElement> for element::XmlElement {
	fn from(value: self::XmlElement) -> Self {
		Self { data: value.xml }
	}
}

impl From<element::XmlElement> for self::XmlElement {
	fn from(value: element::XmlElement) -> Self {
		Self { xml: value.data }
	}
}
