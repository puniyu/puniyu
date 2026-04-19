pub mod plugin;

pub struct Service(&'static str);

impl  Service {
    pub const fn new(name: &'static str) -> Self {
        Self(name)
    }
    pub const fn name(&self) -> &'static str {
        self.0
    }
}