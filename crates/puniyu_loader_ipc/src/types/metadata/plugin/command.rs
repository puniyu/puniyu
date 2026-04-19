use puniyu_command::{ArgMode, ArgType, Permission};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ArgMetaData {
	pub name: String,
	pub arg_type: ArgType,
	pub mode: ArgMode,
	pub required: bool,
	pub description: Option<String>,
}


#[derive(Debug, Deserialize)]
pub struct CommandMetaData {
	pub name: String,
	pub description: Option<String>,
	#[serde(default)]
	pub args: Vec<ArgMetaData>,
	#[serde(default = "default_priority")]
	pub priority: u32,
	#[serde(default)]
	pub alias: Vec<String>,
	#[serde(default)]
	pub permission: Permission,
}

const fn default_priority() -> u32 {
	500
}
