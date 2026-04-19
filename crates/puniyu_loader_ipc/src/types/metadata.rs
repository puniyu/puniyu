mod plugin;
pub use plugin::*;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MetaData {
	pub name: String,
	pub version: String,
	pub description: Option<String>,
	pub author: Vec<String>,
	#[serde(default)]
	pub commands: Vec<CommandMetaData>,
	#[serde(default)]
	pub tasks: Vec<TaskMetaData>,
}
