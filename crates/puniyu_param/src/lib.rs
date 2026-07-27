use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use std::fmt::{self, Display};

/// 参数值。
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum ParamValue {
	/// 字符串值。
	String(String),
	/// 整数值。
	Int(i64),
	/// 浮点数值。
	Float(f64),
	/// 布尔值。
	Bool(bool),
	/// 列表值
	List(Vec<ParamValue>),
}

impl Display for ParamValue {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::String(v) => write!(f, "{}", v),
			Self::Int(v) => write!(f, "{}", v),
			Self::Float(v) => write!(f, "{}", v),
			Self::Bool(v) => write!(f, "{}", v),
			Self::List(v) => {
				write!(f, "[")?;
				for (i, item) in v.iter().enumerate() {
					if i > 0 {
						write!(f, ", ")?;
					}
					write!(f, "{}", item)?;
				}
				write!(f, "]")
			}
		}
	}
}

impl ParamValue {
	/// 获取字符串值。
	pub fn as_str(&self) -> Option<&str> {
		match self {
			Self::String(s) => Some(s.as_str()),
			_ => None,
		}
	}

	/// 获取整数值。
	pub fn as_int(&self) -> Option<i64> {
		match self {
			Self::Int(i) => Some(*i),
			_ => None,
		}
	}

	/// 获取浮点数值。
	pub fn as_float(&self) -> Option<f64> {
		match self {
			Self::Float(f) => Some(*f),
			_ => None,
		}
	}

	/// 获取布尔值。
	pub fn as_bool(&self) -> Option<bool> {
		match self {
			Self::Bool(b) => Some(*b),
			_ => None,
		}
	}

	/// 获取列表值。
	pub fn as_list(&self) -> Option<&[ParamValue]> {
		match self {
			Self::List(v) => Some(v.as_slice()),
			_ => None,
		}
	}
}

impl From<String> for ParamValue {
	fn from(s: String) -> Self {
		Self::String(s)
	}
}

impl From<&str> for ParamValue {
	fn from(s: &str) -> Self {
		Self::String(s.to_string())
	}
}

impl From<i64> for ParamValue {
	fn from(i: i64) -> Self {
		Self::Int(i)
	}
}

impl From<f64> for ParamValue {
	fn from(f: f64) -> Self {
		Self::Float(f)
	}
}

impl From<bool> for ParamValue {
	fn from(b: bool) -> Self {
		Self::Bool(b)
	}
}

impl From<Vec<ParamValue>> for ParamValue {
	fn from(v: Vec<ParamValue>) -> Self {
		Self::List(v)
	}
}

/// 通用参数集合。
///
/// 使用 `IndexMap` 存储有序的 key-value 参数，保持插入顺序。
///
/// # 示例
///
/// ```rust
/// use puniyu_param::{Params, ParamValue};
///
/// let mut params = Params::new();
/// params.push("name", "Alice")
///      .push("age", 25i64)
///      .push("active", true);
///
/// assert_eq!(params.get_as::<String>("name"), Some("Alice".into()));
/// assert_eq!(params.get_as::<i64>("age"), Some(25));
/// assert_eq!(params.get_as::<bool>("active"), Some(true));
/// ```
#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct Params {
	inner: IndexMap<SmolStr, ParamValue>,
}

impl Params {
	/// 创建空参数集合。
	pub fn new() -> Self {
		Self::default()
	}

	/// 按名称获取参数值。
	pub fn get(&self, name: &str) -> Option<&ParamValue> {
		self.inner.get(name)
	}

	/// 按名称获取参数值并转换为目标类型。
	pub fn get_as<T: FromParamValue>(&self, name: &str) -> Option<T> {
		self.get(name).and_then(T::from_param_value)
	}

	/// 添加一个参数。支持链式调用。
	pub fn push(&mut self, name: impl Into<SmolStr>, value: impl Into<ParamValue>) -> &mut Self {
		self.inner.insert(name.into(), value.into());
		self
	}

	/// 是否包含指定名称的参数。
	pub fn contains(&self, name: &str) -> bool {
		self.inner.contains_key(name)
	}

	/// 参数数量。
	pub fn len(&self) -> usize {
		self.inner.len()
	}

	/// 是否没有任何参数。
	pub fn is_empty(&self) -> bool {
		self.inner.is_empty()
	}

	/// 遍历所有参数。
	pub fn iter(&self) -> impl Iterator<Item = (&str, &ParamValue)> {
		self.inner.iter().map(|(k, v)| (k.as_str(), v))
	}

	/// 遍历所有参数名。
	pub fn keys(&self) -> impl Iterator<Item = &str> {
		self.inner.keys().map(|k| k.as_str())
	}

	/// 遍历所有参数值。
	pub fn values(&self) -> impl Iterator<Item = &ParamValue> {
		self.inner.values()
	}
}

impl IntoIterator for Params {
	type Item = (SmolStr, ParamValue);
	type IntoIter = indexmap::map::IntoIter<SmolStr, ParamValue>;

	fn into_iter(self) -> Self::IntoIter {
		self.inner.into_iter()
	}
}

impl<'a> IntoIterator for &'a Params {
	type Item = (&'a str, &'a ParamValue);
	type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'a>;

	fn into_iter(self) -> Self::IntoIter {
		Box::new(self.inner.iter().map(|(k, v)| (k.as_str(), v)))
	}
}

impl FromIterator<(SmolStr, ParamValue)> for Params {
	fn from_iter<I: IntoIterator<Item = (SmolStr, ParamValue)>>(iter: I) -> Self {
		Self { inner: iter.into_iter().collect() }
	}
}


pub trait FromParamValue: Sized {
	fn from_param_value(value: &ParamValue) -> Option<Self>;
}

impl FromParamValue for String {
	fn from_param_value(value: &ParamValue) -> Option<Self> {
		value.as_str().map(String::from)
	}
}

impl FromParamValue for SmolStr {
	fn from_param_value(value: &ParamValue) -> Option<Self> {
		value.as_str().map(SmolStr::new)
	}
}

macro_rules! impl_from_param_value_int {
	($($t:ty),+) => {
		$(
			impl FromParamValue for $t {
				fn from_param_value(value: &ParamValue) -> Option<Self> {
					value.as_int().and_then(|v| v.try_into().ok())
				}
			}
		)+
	};
}

impl_from_param_value_int!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

impl FromParamValue for f64 {
	fn from_param_value(value: &ParamValue) -> Option<Self> {
		value.as_float()
	}
}

impl FromParamValue for f32 {
	fn from_param_value(value: &ParamValue) -> Option<Self> {
		value.as_float().map(|v| v as f32)
	}
}

impl FromParamValue for bool {
	fn from_param_value(value: &ParamValue) -> Option<Self> {
		value.as_bool()
	}
}

impl<T: FromParamValue> FromParamValue for Vec<T> {
	fn from_param_value(value: &ParamValue) -> Option<Self> {
		value.as_list()?.iter().map(T::from_param_value).collect()
	}
}

impl<T: FromParamValue> FromParamValue for Option<T> {
	fn from_param_value(value: &ParamValue) -> Option<Self> {
		Some(T::from_param_value(value))
	}
}
