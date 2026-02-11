use puniyu_version::Version;

#[derive(Debug, Clone, PartialEq)]
pub struct PluginInfo<'p> {
    pub name: &'p str,
    pub author: Option<&'p str>,
    pub description: Option<&'p str>,
    pub version: Option<&'p Version>,
}


#[derive(Debug, Clone)]
pub enum PluginId<'p> {
    Index(u64),
    Name(&'p str),
}
impl From<u64> for PluginId<'_> {
    fn from(value: u64) -> Self {
        Self::Index(value)
    }
}
impl<'p> From<&'p str> for PluginId<'p> {
    fn from(value: &'p str) -> Self {
        Self::Name(value)
    }
}
