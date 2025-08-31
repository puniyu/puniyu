use super::builder::PluginBuilder;

pub struct PluginRegistry {
    pub builder: fn() -> Box<dyn PluginBuilder>,
}
