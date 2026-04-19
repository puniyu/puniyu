mod command;
pub use command::CommandMetaData;
mod task;
pub use task::TaskMetaData;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PluginMetaData {
	pub name: String,
	pub version: String,
	pub description: Option<String>,
	pub author: Vec<String>,
	#[serde(default)]
	pub commands: Vec<CommandMetaData>,
	#[serde(default)]
	pub tasks: Vec<TaskMetaData>,
}
