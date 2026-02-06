#[derive(Debug, Clone)]
pub enum PluginId<'p> {
    Index(u64),
    Name(&'p str),
}
impl From<u64> for PluginId {
    fn from(value: u64) -> Self {
        Self::Index(value)
    }
}
impl From<String> for PluginId {
    fn from(name: String) -> Self {
        Self::Name(name.as_str())
    }
}
impl From<&str> for PluginId {
    fn from(value: &str) -> Self {
        Self::Name(value)
    }
}
