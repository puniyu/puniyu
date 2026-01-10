use std::fmt;

/// 参数值类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArgType {
	String,
	Int,
	Float,
	Bool,
}

impl ArgType {
	pub const fn type_name(&self) -> &'static str {
		match self {
			ArgType::String => "字符串",
			ArgType::Int => "整数",
			ArgType::Float => "浮点数",
			ArgType::Bool => "布尔值",
		}
	}
}

/// 参数模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ArgMode {
	/// 位置参数（按顺序匹配）
	#[default]
	Positional,
	/// 命名参数（需要 --flag）
	Named,
}

#[derive(Debug, Clone)]
pub struct Arg<'a> {
	/// 参数名
	pub name: &'a str,
	/// 参数类型
	pub arg_type: ArgType,
	/// 参数模式
	pub mode: ArgMode,
	/// 是否必须
	pub required: bool,
	/// 描述
	pub description: Option<&'a str>,
}

impl<'a> Default for Arg<'a> {
	fn default() -> Self {
		Self {
			name: "",
			arg_type: ArgType::String,
			mode: ArgMode::Positional,
			required: false,
			description: None,
		}
	}
}

impl<'a> Arg<'a> {
	pub fn new(name: &'a str) -> Self {
		Self { name, ..Default::default() }
	}

	pub fn string(name: &'a str) -> Self {
		Self::new(name).with_type(ArgType::String)
	}

	pub fn int(name: &'a str) -> Self {
		Self::new(name).with_type(ArgType::Int)
	}

	pub fn float(name: &'a str) -> Self {
		Self::new(name).with_type(ArgType::Float)
	}

	pub fn bool(name: &'a str) -> Self {
		Self::new(name).with_type(ArgType::Bool)
	}

	pub fn with_type(mut self, arg_type: ArgType) -> Self {
		self.arg_type = arg_type;
		self
	}

	pub fn required(mut self) -> Self {
		self.required = true;
		self
	}

	pub fn optional(mut self) -> Self {
		self.required = false;
		self
	}

	pub fn description(mut self, desc: &'a str) -> Self {
		self.description = Some(desc);
		self
	}

	/// 设置为位置参数（默认）
	pub fn positional(mut self) -> Self {
		self.mode = ArgMode::Positional;
		self
	}

	/// 设置为命名参数（需要 --flag）
	pub fn named(mut self) -> Self {
		self.mode = ArgMode::Named;
		self
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArgValue {
	String(String),
	Int(i64),
	Float(f64),
	Bool(bool),
}

impl ArgValue {
	pub fn as_str(&self) -> Option<&str> {
		match self {
			ArgValue::String(s) => Some(s),
			_ => None,
		}
	}

	pub fn as_int(&self) -> Option<i64> {
		match self {
			ArgValue::Int(i) => Some(*i),
			_ => None,
		}
	}

	pub fn as_float(&self) -> Option<f64> {
		match self {
			ArgValue::Float(f) => Some(*f),
			_ => None,
		}
	}

	pub fn as_bool(&self) -> Option<bool> {
		match self {
			ArgValue::Bool(b) => Some(*b),
			_ => None,
		}
	}
}

impl fmt::Display for ArgValue {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			ArgValue::String(s) => write!(f, "{}", s),
			ArgValue::Int(i) => write!(f, "{}", i),
			ArgValue::Float(fl) => write!(f, "{}", fl),
			ArgValue::Bool(b) => write!(f, "{}", b),
		}
	}
}

impl From<String> for ArgValue {
	fn from(s: String) -> Self {
		ArgValue::String(s)
	}
}

impl From<&str> for ArgValue {
	fn from(s: &str) -> Self {
		ArgValue::String(s.to_string())
	}
}

impl From<i64> for ArgValue {
	fn from(i: i64) -> Self {
		ArgValue::Int(i)
	}
}

impl From<f64> for ArgValue {
	fn from(f: f64) -> Self {
		ArgValue::Float(f)
	}
}

impl From<bool> for ArgValue {
	fn from(b: bool) -> Self {
		ArgValue::Bool(b)
	}
}
