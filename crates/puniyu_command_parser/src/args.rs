use crate::error::Error;
use puniyu_command_types::{Arg, ArgMode, ArgValue, ArgType};
use smol_str::SmolStr;
use std::collections::HashMap;

pub(crate) fn parse_args(
	tokens: &[String],
	arg_defs: &[Arg<'_>],
) -> Result<HashMap<SmolStr, Vec<ArgValue>>, Error> {
	let mut result: HashMap<SmolStr, Vec<ArgValue>> = HashMap::new();
	let mut positional_index = 0;
	let mut i = 0;

	while i < tokens.len() {
		let token = &tokens[i];

		if let Some(option) = parse_option(token) {
			let def = find_named_def(arg_defs, option)?;
			let name: SmolStr = def.name.as_ref().into();
			let is_next_value = i + 1 < tokens.len() && !is_option(&tokens[i + 1]);
			let value = consume_value(&mut i, is_next_value, tokens, &def.arg_type, &name)?;
			result.entry(name).or_default().push(value);
		} else {
			let positional_defs: Vec<_> =
				arg_defs.iter().filter(|a| a.mode == ArgMode::Positional).collect();
			let def = positional_defs
				.get(positional_index)
				.or(positional_defs.last())
				.ok_or_else(|| Error::TooManyValues { arg_name: token.clone() })?;

			let name: SmolStr = def.name.as_ref().into();
			let value = parse_value(token, &def.arg_type, &name)?;
			result.entry(name).or_default().push(value);
			positional_index += 1;
		}

		i += 1;
	}

	for def in arg_defs {
		if def.required {
			let name: SmolStr = def.name.as_ref().into();
			if !result.contains_key(&name) {
				return Err(Error::MissingRequired { arg_name: name.to_string() });
			}
		}
	}

	Ok(result)
}

fn parse_option(token: &str) -> Option<&str> {
	if let Some(long) = token.strip_prefix("--") {
		Some(long)
	} else if token.starts_with('-') && token.len() == 2 {
		Some(&token[1..])
	} else {
		None
	}
}

fn is_option(token: &str) -> bool {
	token.starts_with('-')
}

fn find_named_def<'a>(arg_defs: &'a [Arg<'_>], option: &str) -> Result<&'a Arg<'a>, Error> {
	arg_defs
		.iter()
		.find(|a| {
			a.mode == ArgMode::Named
				&& (a.long.as_deref() == Some(option)
					|| a.short.map(|c| c.to_string()) == Some(option.to_string())
					|| a.name.as_ref() == option)
		})
		.ok_or_else(|| Error::UnknownArgument { arg_name: option.to_string() })
}

fn consume_value(
	i: &mut usize,
	is_next_value: bool,
	tokens: &[String],
	elem_type: &ArgType,
	name: &SmolStr,
) -> Result<ArgValue, Error> {
	if *elem_type == ArgType::Bool && !is_next_value {
		return Ok(ArgValue::Bool(true));
	}

	if !is_next_value {
		return Err(Error::MissingRequired { arg_name: name.to_string() });
	}

	*i += 1;
	parse_value(&tokens[*i], elem_type, name)
}

fn parse_value(raw: &str, elem_type: &ArgType, arg_name: &str) -> Result<ArgValue, Error> {
	match elem_type {
		ArgType::String => Ok(ArgValue::String(raw.to_string())),
		ArgType::Int => {
			raw.parse::<i64>().map(ArgValue::Int).map_err(|_| invalid(arg_name, "integer", raw))
		}
		ArgType::Float => {
			raw.parse::<f64>().map(ArgValue::Float).map_err(|_| invalid(arg_name, "float", raw))
		}
		ArgType::Bool => match raw {
			"true" | "1" | "yes" => Ok(ArgValue::Bool(true)),
			"false" | "0" | "no" => Ok(ArgValue::Bool(false)),
			_ => Err(invalid(arg_name, "boolean", raw)),
		},
	}
}

fn invalid(arg_name: &str, expected: &str, raw: &str) -> Error {
	Error::InvalidValue {
		arg_name: arg_name.to_string(),
		expected_type: expected.to_string(),
		raw_value: raw.to_string(),
	}
}
