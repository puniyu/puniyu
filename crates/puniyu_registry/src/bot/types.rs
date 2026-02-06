#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BotId<'b> {
    Index(u64),
    SelfId(&'b str),
}

impl From<u64> for BotId {
    fn from(index: u64) -> Self {
        Self::Index(index)
    }
}

impl<'b> From<&str> for BotId<'b> {
    fn from(self_id: &'b str) -> Self {
        Self::SelfId(self_id)
    }
}

impl From<String> for BotId {
    fn from(self_id: String) -> Self {
        Self::SelfId(self_id.as_str())
    }
}