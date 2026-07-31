#[derive(Debug, Clone)]
pub enum PluginId<'p> {
	Index(u64),
	Name(&'p str),
}

impl From<u64> for PluginId<'_> {
	fn from(id: u64) -> Self {
		Self::Index(id)
	}
}

impl<'p> From<&'p str> for PluginId<'p> {
	fn from(name: &'p str) -> Self {
		Self::Name(name)
	}
}