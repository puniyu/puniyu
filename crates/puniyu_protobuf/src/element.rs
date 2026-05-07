use puniyu_element;

pub mod receive;
pub mod send;

include!(concat!(env!("OUT_DIR"), "/puniyu.element.rs"));

impl_enum_from!(ElementType => puniyu_element::ElementType {
	At,
	Reply,
	Text,
	Image,
	File,
	Record,
	Video,
	Face,
	Json,
	Xml,
});

macro_rules! impl_element_from {
	(bi, $left:ty, $right:ty, { $($field:ident),+ $(,)? }) => {
		impl From<$left> for $right {
			fn from(value: $left) -> Self {
				Self {
					$($field: value.$field),+
				}
			}
		}

		impl From<$right> for $left {
			fn from(value: $right) -> Self {
				Self {
					$($field: value.$field),+
				}
			}
		}
	};

	(map, $from:ty => $to:ty, $value:ident, { $($field:ident : $expr:expr),+ $(,)? }) => {
		impl From<$from> for $to {
			fn from($value: $from) -> Self {
				Self {
					$($field: $expr),+
				}
			}
		}
	};

	(map<$($generics:tt),+>, $from:ty => $to:ty, $value:ident, { $($field:ident : $expr:expr),+ $(,)? }) => {
		impl<$($generics),+> From<$from> for $to {
			fn from($value: $from) -> Self {
				Self {
					$($field: $expr),+
				}
			}
		}
	};

	(enum, $from:ty => $to:ty, { $( $from_variant:path => $to_variant:path ),+ $(,)? }) => {
		impl From<$from> for $to {
			fn from(value: $from) -> Self {
				match value {
					$( $from_variant(inner) => $to_variant(inner.into()), )+
				}
			}
		}
	};

	(enum<$($generics:tt),+>, $from:ty => $to:ty, { $( $from_variant:path => $to_variant:path ),+ $(,)? }) => {
		impl<$($generics),+> From<$from> for $to {
			fn from(value: $from) -> Self {
				match value {
					$( $from_variant(inner) => $to_variant(inner.into()), )+
				}
			}
		}
	};

	(oneof, $from:ty => $to:ty, { $( $from_variant:path => $to_variant:path ),+ $(,)? }, none = $none_message:expr) => {
		impl From<$from> for $to {
			fn from(value: $from) -> Self {
				match value.element {
					$( Some($from_variant(inner)) => $to_variant(inner.into()), )+
					None => panic!($none_message),
				}
			}
		}
	};

	(oneof<$($generics:tt),+>, $from:ty => $to:ty, $value:ident, { $( $from_variant:path => $to_variant:path ),+ $(,)? }, none = $none_message:expr) => {
		impl<$($generics),+> From<$from> for $to {
			fn from($value: $from) -> Self {
				match $value.element.as_ref() {
					$( Some($from_variant(inner)) => $to_variant(inner.into()), )+
					None => panic!($none_message),
				}
			}
		}
	};
}

macro_rules! impl_vec_element_from {
	(bi, $internal_type:ty, $proto_type:ty, $proto_field:ident) => {
		impl From<Vec<$internal_type>> for $proto_type {
			fn from(elements: Vec<$internal_type>) -> Self {
				Self { $proto_field: elements.into_iter().map(Into::into).collect() }
			}
		}

		impl From<$proto_type> for Vec<$internal_type> {
			fn from(elements: $proto_type) -> Self {
				elements.$proto_field.into_iter().map(Into::into).collect()
			}
		}
	};

	(map<$($generics:tt),+>, $from:ty => Vec<$internal_type:ty>, $value:ident, $iter:expr) => {
		impl<$($generics),+> From<$from> for Vec<$internal_type> {
			fn from($value: $from) -> Self {
				($iter).collect()
			}
		}
	};
}

pub(crate) use impl_element_from;
pub(crate) use impl_vec_element_from;
