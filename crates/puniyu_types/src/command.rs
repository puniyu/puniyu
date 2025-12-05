use crate::context::{BotContext, MessageContext};
use async_trait::async_trait;

/// 命令处理动作
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandlerAction {
	/// 处理完成，停止传播
	Done,
	/// 继续传播给其他处理器
	Continue,
}

impl HandlerAction {
	pub const fn done() -> HandlerResult {
		Ok(HandlerAction::Done)
	}
	pub const fn r#continue() -> HandlerResult {
		Ok(HandlerAction::Continue)
	}
}

impl From<()> for HandlerAction {
	fn from(_: ()) -> Self {
		HandlerAction::Done
	}
}

/// 命令处理结果
pub type HandlerResult = Result<HandlerAction, Box<dyn std::error::Error + Send + Sync>>;

/// 参数值类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArgType {
    String,
    Int,
    Float,
    Bool,
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
pub struct Arg {
    /// 参数名
    pub name: &'static str,
    /// 参数类型
    pub arg_type: ArgType,
    /// 参数模式
    pub mode: ArgMode,
    /// 是否必须
    pub required: bool,
    /// 默认值
    pub default: Option<ArgValue>,
    /// 描述
    pub description: Option<&'static str>,
}

impl Default for Arg {
    fn default() -> Self {
        Self {
            name: "",
            arg_type: ArgType::String,
            mode: ArgMode::Positional,
            required: false,
            default: None,
            description: None,
        }
    }
}

impl Arg {
    pub fn new(name: &'static str) -> Self {
        Self { name, ..Default::default() }
    }

    pub fn string(name: &'static str) -> Self {
        Self::new(name).with_type(ArgType::String)
    }

    pub fn int(name: &'static str) -> Self {
        Self::new(name).with_type(ArgType::Int)
    }

    pub fn float(name: &'static str) -> Self {
        Self::new(name).with_type(ArgType::Float)
    }

    pub fn bool(name: &'static str) -> Self {
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

    pub fn default_value(mut self, value: ArgValue) -> Self {
        self.default = Some(value);
        self
    }

    pub fn description(mut self, desc: &'static str) -> Self {
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

    pub fn into_string(self) -> Option<String> {
        match self {
            ArgValue::String(s) => Some(s),
            _ => None,
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

/// 权限等级
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Permission {
    /// 所有人可用（默认）
    #[default]
    All,
    /// 仅主人可用
    Master,
}

impl std::str::FromStr for Permission {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "master" => Permission::Master,
            _ => Permission::All,
        })
    }
}

/// 命令
#[derive(Debug, Clone)]
pub struct Command {
    pub name: &'static str,
    pub description: Option<&'static str>,
    pub args: Vec<Arg>,
    pub rank: u64,
    /// 自定义前缀，None 表示使用全局前缀
    pub prefix: Option<String>,
    /// 命令别名
    pub alias: Option<Vec<&'static str>>,
    /// 权限等级
    pub permission: Permission,
}

#[async_trait]
pub trait CommandBuilder: Send + Sync + 'static {
    /// 命令名称
    fn name(&self) -> &'static str;

	/// 描述
	fn description(&self) -> Option<&'static str>;

	/// 参数列表
	fn args(&self) -> Vec<Arg>;

	/// 优先级
	fn rank(&self) -> u64;

	/// 命令别名
	fn alias(&self) -> Option<Vec<&'static str>> {
		None
	}

	/// 权限等级
	fn permission(&self) -> Permission {
		Permission::All
	}

	/// 执行的函数
	async fn run(&self, bot: &BotContext, ev: &MessageContext) -> HandlerResult;
}