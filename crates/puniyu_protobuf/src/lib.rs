macro_rules! impl_enum_from {
	($left:path => $right:path { $($variant:ident),+ $(,)? }) => {
		impl_enum_from!($left => $right { $($variant => $variant),+ });
	};

	($left:path => $right:path { $($left_variant:ident => $right_variant:ident),+ $(,)? }) => {
		impl From<$left> for $right {
			fn from(value: $left) -> Self {
				match value {
					$(<$left>::$left_variant => <$right>::$right_variant,)+
				}
			}
		}

		impl From<$right> for $left {
			fn from(value: $right) -> Self {
				match value {
					$(<$right>::$right_variant => <$left>::$left_variant,)+
				}
			}
		}
	};
}


pub mod account;
pub mod adapter;
pub mod bot;
pub mod contact;
pub mod element;
pub mod event;
pub mod sender;
pub mod version;