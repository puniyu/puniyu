use crate::plugin::task::builder::TaskBuilder;

pub mod builder;
pub(crate) mod manger;
pub mod registry;

#[derive(Debug, Clone)]
pub struct Task {
    pub name: &'static str,
    pub cron: &'static str,
}
