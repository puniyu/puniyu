use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScopeId(u64);

impl ScopeId {
	pub(crate) const fn new(value: u64) -> Self {
		Self(value)
	}

	pub const fn root() -> Self {
		Self(0)
	}

	pub const fn as_u64(&self) -> u64 {
		self.0
	}
}

impl fmt::Display for ScopeId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}
