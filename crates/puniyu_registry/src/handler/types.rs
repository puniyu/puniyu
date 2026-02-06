pub(crate) enum HandlerId<'h> {
	Index(u64),
	Name(&'h str),
}

impl From<u64> for HandlerId<'_> {
	fn from(index: u64) -> Self {
		Self::Index(index)
	}
}

impl From<&str> for HandlerId<'_> {
	fn from(name: &str) -> Self {
		Self::Name(name)
	}
}

impl From<String> for HandlerId<'_>{
    fn from(name: String) -> Self {
        Self::Name(name.as_str())
    }
}
