use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Permission {
	/// 所有人可用（默认）
	#[default]
	All,
	/// 仅主人可用
	Master,
}

impl std::str::FromStr for Permission {
	type Err = std::convert::Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s.to_lowercase().as_str() {
			"master" => Permission::Master,
			_ => Permission::All,
		})
	}
}

impl fmt::Display for Permission {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Permission::All => write!(f, "all"),
			Permission::Master => write!(f, "master"),
		}
	}
}
