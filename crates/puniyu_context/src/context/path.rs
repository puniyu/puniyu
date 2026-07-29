use std::ops::Deref;

pub struct PathConetxt {
	inner: puniyu_path::Path,
}

impl PathConetxt {
    pub fn new(path: puniyu_path::Path) -> Self {
        Self { inner: path }
    }
}


impl Deref for PathConetxt {
    type Target = puniyu_path::Path;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}