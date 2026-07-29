use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ServiceName(SmolStr);

impl ServiceName {
	pub const fn new(name: &'static str) -> Self {
		Self(SmolStr::new_static(name))
	}

	pub fn as_str(&self) -> &str {
		self.0.as_str()
	}
}

impl AsRef<str> for ServiceName {
	fn as_ref(&self) -> &str {
		&self.0
	}
}


