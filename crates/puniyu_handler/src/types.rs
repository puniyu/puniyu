pub enum HandlerId<'h> {
    Index(u64),
    Name(&'h str),
}

impl From<u64> for HandlerId<'_> {
    fn from(index: u64) -> Self {
        Self::Index(index)
    }
}

impl<'h> From<&'h str> for HandlerId<'h> {
    fn from(name: &'h str) -> Self {
        Self::Name(name)
    }
}
