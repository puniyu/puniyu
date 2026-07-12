#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SourceType {
	Plugin(u64),
	Adapter(u64),
}

impl SourceType {
	pub fn is_plugin(&self) -> bool {
		matches!(self, SourceType::Plugin(_))
	}
	pub fn is_adapter(&self) -> bool {
		matches!(self, SourceType::Adapter(_))
	}
}