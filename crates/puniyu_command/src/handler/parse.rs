use clap::builder::ValueParser;
use puniyu_types::command::{Arg, ArgMode, ArgType, ArgValue};
use std::collections::HashMap;

struct ClapArg<'a>(&'a Arg);

impl From<ClapArg<'_>> for clap::Arg {
	fn from(ClapArg(arg): ClapArg<'_>) -> Self {
		let mut clap_arg = clap::Arg::new(arg.name);

		clap_arg = match arg.arg_type {
			ArgType::String => clap_arg.value_parser(ValueParser::string()),
			ArgType::Int => clap_arg.value_parser(clap::value_parser!(i64)),
			ArgType::Float => clap_arg.value_parser(clap::value_parser!(f64)),
			ArgType::Bool => clap_arg
				.value_parser(clap::value_parser!(bool))
				.num_args(0..=1)
				.default_missing_value("true"),
		};

		clap_arg = match arg.mode {
			ArgMode::Positional => clap_arg.required(arg.required),
			ArgMode::Named => clap_arg.long(arg.name).required(arg.required),
		};

		if let Some(ref default) = arg.default {
			clap_arg = clap_arg.default_value(match default {
				ArgValue::String(s) => s.clone(),
				ArgValue::Int(i) => i.to_string(),
				ArgValue::Float(f) => f.to_string(),
				ArgValue::Bool(b) => b.to_string(),
			});
		}

		clap_arg
	}
}

pub(crate) struct ArgParser;

impl ArgParser {
	fn create_clap_command(name: &str, args: &[Arg]) -> clap::Command {
		let mut cmd = clap::Command::new(name.to_string())
			.no_binary_name(true)
			.disable_help_flag(true)
			.disable_version_flag(true);

		for arg in args {
			cmd = cmd.arg(clap::Arg::from(ClapArg(arg)));
		}
		cmd
	}

	fn format_error(e: clap::Error, args: &[Arg]) -> String {
		use clap::error::{ContextKind, ErrorKind};

		let get_arg_name = |e: &clap::Error| {
			e.get(ContextKind::InvalidArg)
				.map(|v| v.to_string())
				.unwrap_or_default()
				.trim_matches(|c| c == '<' || c == '>' || c == '-')
				.to_string()
		};

		let get_type_name = |arg_name: &str| {
			args.iter()
				.find(|a| a.name == arg_name)
				.map(|a| match a.arg_type {
					ArgType::String => "字符串",
					ArgType::Int => "整数",
					ArgType::Float => "浮点数",
					ArgType::Bool => "布尔值",
				})
				.unwrap_or("有效值")
		};

		match e.kind() {
			ErrorKind::InvalidValue | ErrorKind::ValueValidation => {
				let arg_name = get_arg_name(&e);
				format!("参数 {} 输入无效，请提供一个{}", arg_name, get_type_name(&arg_name))
			}
			ErrorKind::UnknownArgument => {
				format!("未知参数: {}", get_arg_name(&e))
			}
			ErrorKind::TooManyValues => {
				format!("参数 {} 提供了过多的值", get_arg_name(&e))
			}
			ErrorKind::TooFewValues => {
				format!("参数 {} 提供的值不足", get_arg_name(&e))
			}
			ErrorKind::MissingRequiredArgument => {
				format!("缺少必需参数: {}", get_arg_name(&e))
			}
			_ => e.to_string(),
		}
	}

	pub(crate) fn parse(
		command_name: &str,
		args: &[String],
		arg_defs: &[Arg],
	) -> Result<HashMap<String, ArgValue>, String> {
		let cmd = Self::create_clap_command(command_name, arg_defs);

		let matches =
			cmd.try_get_matches_from(args).map_err(|e| Self::format_error(e, arg_defs))?;

		let mut result = HashMap::new();
		for arg_def in arg_defs {
			if let Some(value) = Self::get_value(&matches, arg_def) {
				result.insert(arg_def.name.to_string(), value);
			}
		}

		Ok(result)
	}

	fn get_value(matches: &clap::ArgMatches, arg_def: &Arg) -> Option<ArgValue> {
		match arg_def.arg_type {
			ArgType::String => {
				matches.get_one::<String>(arg_def.name).map(|s| ArgValue::String(s.clone()))
			}
			ArgType::Int => matches.get_one::<i64>(arg_def.name).map(|i| ArgValue::Int(*i)),
			ArgType::Float => matches.get_one::<f64>(arg_def.name).map(|f| ArgValue::Float(*f)),
			ArgType::Bool => matches.get_one::<bool>(arg_def.name).map(|b| ArgValue::Bool(*b)),
		}
	}
}
