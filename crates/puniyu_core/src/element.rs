use std::{fmt::Debug, hash::Hash};

pub trait Element: Send + Sync {
	type ElementType: Copy;

	/// 元素类型
	fn r#type(&self) -> Self::ElementType;
}

impl<T: Element + ?Sized> Element for &T {
	type ElementType = T::ElementType;
	fn r#type(&self) -> Self::ElementType {
		(**self).r#type()
	}
}


impl<T: Copy + PartialEq> PartialEq
	for dyn Element<ElementType = T>
{
	fn eq(&self, other: &Self) -> bool {
		self.r#type() == other.r#type()
	}
}

impl<T: Copy + PartialEq> Eq
	for dyn Element<ElementType = T>
{
}

impl<T: Copy + Hash> Hash
	for dyn Element<ElementType = T>
{
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.r#type().hash(state);
	}
}

impl<T: Copy + Debug> Debug
	for dyn Element<ElementType = T>
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Element").field("type", &self.r#type()).finish()
	}
}