#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BotId<'b> {
    Index(u64),
    SelfId(&'b str),
}

impl<'b> From<u64> for BotId<'b> {
    fn from(index: u64) -> Self {
        Self::Index(index)
    }
}

impl<'b> From<&'b str> for BotId<'b> {
    fn from(name: &'b str) -> Self {
        Self::SelfId(name)
    }
}