pub(crate) enum AdapterId<'a> {
    Index(u64),
    Name(&'a str),
}

impl From<u64> for AdapterId {
    fn from(index: u64) -> Self {
        Self::Index(index)
    }
}

impl<'b> From<&str> for AdapterId<'b> {
    fn from(self_id: &'b str) -> Self {
        Self::Name(self_id)
    }
}

impl From<String> for AdapterId {
    fn from(self_id: String) -> Self {
        Self::Name(self_id.as_str())
    }
}