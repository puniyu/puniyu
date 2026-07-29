pub struct AdapterContext {
    pub(crate) path: puniyu_path::Path
}

impl AdapterContext {
    pub fn path(&self) -> &puniyu_path::Path {
        &self.path
    }
}
