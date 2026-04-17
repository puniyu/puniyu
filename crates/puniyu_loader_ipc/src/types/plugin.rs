use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct ProcessMeta {
	pub name: String,
	pub version: String,
	pub description: Option<String>,
	pub author: Vec<String>
}