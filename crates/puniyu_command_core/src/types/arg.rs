//! 命令参数类型定义

use serde::{Deserialize, Serialize};
use std::fmt;

/// 参数值类型
///
/// 定义命令参数支持的数据类型。
///
/// # 示例
///
/// ```rust
/// use puniyu_command_core::ArgType;
///
/// let string_type = ArgType::String;
/// let int_type = ArgType::Int;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum ArgType {
	/// 字符串类型
	String,
	/// 整数类型
	Int,
	/// 浮点数类型
	Float,
	/// 布尔值类型
	Bool,
}

/// 参数模式
///
/// 定义参数的匹配方式。
///
/// # 示例
///
/// ```rust
/// use puniyu_command_core::ArgMode;
///
/// // 位置参数：按顺序匹配
/// let positional = ArgMode::Positional;
///
/// // 命名参数：需要 --flag
/// let named = ArgMode::Named;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ArgMode {
	/// 位置参数（按顺序匹配）
	#[default]
	Positional,
	/// 命名参数（需要 --flag）
	Named,
}

/// 命令参数定义
///
/// 定义命令接受的参数，包括名称、类型、是否必需等信息。
///
/// # 示例
///
/// ## 创建字符串参数
///
/// ```rust
/// use puniyu_command_core::Arg;
///
/// let arg = Arg::string("name")
///     .required()
///     .description("用户名称");
/// ```
///
/// ## 创建整数参数
///
/// ```rust
/// use puniyu_command_core::Arg;
///
/// let arg = Arg::int("count")
///     .optional()
///     .description("数量");
/// ```
///
/// ## 创建命名参数
///
/// ```rust
/// use puniyu_command_core::Arg;
///
/// let arg = Arg::bool("verbose")
///     .named()
///     .description("详细输出");
/// ```
#[derive(Debug, Clone, PartialEq)]
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
	/// 创建新的参数定义
	///
	/// # 参数
	///
	/// - `name` - 参数名称
	pub fn new(name: &'a str) -> Self {
		Self { name, ..Default::default() }
	}

	/// 创建字符串类型参数
	pub fn string(name: &'a str) -> Self {
		Self::new(name).with_type(ArgType::String)
	}

	/// 创建整数类型参数
	pub fn int(name: &'a str) -> Self {
		Self::new(name).with_type(ArgType::Int)
	}

	/// 创建浮点数类型参数
	pub fn float(name: &'a str) -> Self {
		Self::new(name).with_type(ArgType::Float)
	}

	/// 创建布尔值类型参数
	pub fn bool(name: &'a str) -> Self {
		Self::new(name).with_type(ArgType::Bool)
	}

	/// 设置参数类型
	pub fn with_type(mut self, arg_type: ArgType) -> Self {
		self.arg_type = arg_type;
		self
	}

	/// 设置为必需参数
	pub fn required(mut self) -> Self {
		self.required = true;
		self
	}

	/// 设置为可选参数
	pub fn optional(mut self) -> Self {
		self.required = false;
		self
	}

	/// 设置参数描述
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

/// 参数值
///
/// 表示命令参数的实际值。
///
/// # 示例
///
/// ```rust
/// use puniyu_command_core::ArgValue;
///
/// // 从不同类型创建
/// let str_val = ArgValue::from("hello");
/// let int_val = ArgValue::from(42i64);
/// let float_val = ArgValue::from(3.14f64);
/// let bool_val = ArgValue::from(true);
///
/// // 提取值
/// if let Some(s) = str_val.as_str() {
///     println!("字符串: {}", s);
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum ArgValue {
	/// 字符串值
	String(String),
	/// 整数值
	Int(i64),
	/// 浮点数值
	Float(f64),
	/// 布尔值
	Bool(bool),
}

impl ArgValue {
	/// 尝试获取字符串值
	pub fn as_str(&self) -> Option<&str> {
		match self {
			ArgValue::String(s) => Some(s),
			_ => None,
		}
	}

	/// 尝试获取整数值
	pub fn as_int(&self) -> Option<i64> {
		match self {
			ArgValue::Int(i) => Some(*i),
			_ => None,
		}
	}

	/// 尝试获取浮点数值
	pub fn as_float(&self) -> Option<f64> {
		match self {
			ArgValue::Float(f) => Some(*f),
			_ => None,
		}
	}

	/// 尝试获取布尔值
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
		Self::Int(i)
	}
}

impl From<u64> for ArgValue {
	fn from(u: u64) -> Self {
		Self::Int(u as i64)
	}
}
impl From<f64> for ArgValue {
	fn from(f: f64) -> Self {
		Self::Float(f)
	}
}

impl From<bool> for ArgValue {
	fn from(b: bool) -> Self {
		Self::Bool(b)
	}
}
