use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ServiceName(SmolStr);

impl ServiceName {
	pub const fn new_static(name: &'static str) -> Self {
		Self(SmolStr::new_static(name))
	}

	pub fn new(name: impl Into<SmolStr>) -> Self {
		Self(name.into())
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

impl std::fmt::Display for ServiceName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.fmt(f)
	}
}


