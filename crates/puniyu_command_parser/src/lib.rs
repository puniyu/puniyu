//! # puniyu_command_parser
//!
//! 命令文本解析器，负责剥离别名/前缀、分词、参数类型化。
//!
//! ## 两阶段解析
//!
//! 1. `parse()` — 纯文本处理：剥离 alias/prefix → shlex 分词
//! 2. `into_args(arg_defs)` — 传入命令参数定义，得到类型化参数
//!
//! ## 示例
//!
//! ```rust,no_run
//! use puniyu_command_parser::CommandParser;
//! use puniyu_command_types::Arg;
//!
//! let parser = CommandParser::builder()
//!     .alias("@bot")
//!     .prefix("!")
//!     .build();
//!
//! let result = parser.parse("@bot !greet Alice")?;
//! assert_eq!(result.command(), "greet");
//!
//! let args = result.into_args(&[Arg::string("name").positional().required()])?;
//! # Ok::<(), puniyu_command_parser::Error>(())
//! ```

mod args;
mod error;

#[doc(inline)]
pub use error::Error;

use bon::Builder;
use puniyu_command_types::Arg;
use smol_str::SmolStr;
use std::collections::HashMap;

/// 命令文本解析器。
#[derive(Default, Builder)]
pub struct CommandParser {
	#[builder(field)]
	aliases: Vec<String>,
	#[builder(field)]
	prefixes: Vec<String>,
}

impl<S: command_parser_builder::State> CommandParserBuilder<S> {
	/// 添加一个 bot 别名。
	pub fn alias(mut self, alias: impl Into<String>) -> Self {
		self.aliases.push(alias.into());
		self
	}

	/// 添加一个命令前缀。
	pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
		self.prefixes.push(prefix.into());
		self
	}
}

impl CommandParser {
	/// 创建默认解析器
	pub fn new() -> Self {
		Self::default()
	}

	/// 解析命令文本，返回命令名和原始 token。
	pub fn parse(self, input: &str) -> Result<ParseResult, Error> {
		let text = Self::strip_patterns(input, &self.aliases, &self.prefixes);
		let text = text.trim();
		let tokens = shlex::split(text).ok_or(Error::EmptyInput)?;
		if tokens.is_empty() || tokens[0].is_empty() {
			return Err(Error::EmptyInput);
		}

		let command = tokens[0].clone();
		let args = tokens[1..].to_vec();
		Ok(ParseResult { command, tokens: args })
	}

	fn strip_patterns<'t>(text: &'t str, aliases: &[String], prefixes: &[String]) -> &'t str {
		let text = Self::strip_first(text, aliases);
		Self::strip_first(text, prefixes)
	}

	fn strip_first<'t>(text: &'t str, patterns: &[String]) -> &'t str {
		patterns
			.iter()
			.find_map(|p| {
				if p.is_empty() {
					None
				} else {
					text.strip_prefix(p.as_str())
				}
			})
			.map(str::trim_start)
			.unwrap_or(text)
	}
}

/// 命令解析结果。
pub struct ParseResult {
	command: String,
	tokens: Vec<String>,
}

impl ParseResult {
	/// 获取命令名称。
	pub fn command(&self) -> &str {
		&self.command
	}

	/// 获取原始 token 列表。
	pub fn tokens(&self) -> &[String] {
		&self.tokens
	}

	/// 根据参数定义，将 token 列表解析为类型化的参数映射。
	pub fn into_args(self, arg: &[Arg<'_>]) -> Result<HashMap<SmolStr, Vec<puniyu_command_types::ArgValue>>, Error> {
		args::parse_args(&self.tokens, arg)
	}
}
