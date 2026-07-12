use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

const fn default_prefix() -> Option<SmolStr> {
	Some(SmolStr::new_static("!"))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandConfig {
	#[serde(default = "default_prefix", skip_serializing_if = "Option::is_none")]
	prefix: Option<SmolStr>,
}

impl CommandConfig {
	pub fn prefix(&self) -> Option<&str> {
		self.prefix.as_deref()
	}
}

impl Default for CommandConfig {
	#[inline]
	fn default() -> Self {
		Self { prefix: default_prefix() }
	}
}
