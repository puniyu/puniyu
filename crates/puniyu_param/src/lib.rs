use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use std::fmt::{self, Display};
use std::marker::PhantomData;
use std::ops::Deref;

/// 参数值。
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum ParamValue {
	/// 空值
	Empty,
	/// 字符串值。
	String(String),
	/// 整数值。
	Int(i64),
	/// 浮点数值。
	Float(f64),
	/// 布尔值。
	Bool(bool),
	/// 列表值。
	List(Vec<ParamValue>),
	/// 映射值。
	Map(IndexMap<SmolStr, ParamValue>),
}

impl Display for ParamValue {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Empty => write!(f, "()"),
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
			Self::Map(v) => {
				write!(f, "{{")?;
				for (i, (k, val)) in v.iter().enumerate() {
					if i > 0 {
						write!(f, ", ")?;
					}
					write!(f, "{}: {}", k, val)?;
				}
				write!(f, "}}")
			}
		}
	}
}

impl ParamValue {
	pub fn as_empty(&self) -> Option<()> {
		match self {
			Self::Empty => Some(()),
			_ => None,
		}
	}
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

	/// 获取映射值。
	pub fn as_map(&self) -> Option<&IndexMap<SmolStr, ParamValue>> {
		match self {
			Self::Map(m) => Some(m),
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

impl From<IndexMap<SmolStr, ParamValue>> for ParamValue {
	fn from(m: IndexMap<SmolStr, ParamValue>) -> Self {
		Self::Map(m)
	}
}


/// 从 [`ParamValue`] 转换为目标类型的 trait。
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

impl FromParamValue for IndexMap<SmolStr, ParamValue> {
	fn from_param_value(value: &ParamValue) -> Option<Self> {
		value.as_map().cloned()
	}
}

/// 编译期类型安全的参数键。
///
/// 键在编译时绑定名称和期望类型，消除运行时类型错误。
pub struct ParamKey<T: FromParamValue> {
	name: &'static str,
	_phantom: PhantomData<T>,
}

impl<T: FromParamValue> ParamKey<T> {
	pub const fn new(name: &'static str) -> Self {
		Self { name, _phantom: PhantomData }
	}
}

impl<T: FromParamValue> Deref for ParamKey<T> {
	type Target = str;
	fn deref(&self) -> &str {
		self.name
	}
}
