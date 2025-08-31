use super::builder::TaskBuilder;

pub struct TaskRegistry {
    /// 插件名称
    pub plugin_name: &'static str,
    /// 任务构建器
    pub builder: fn() -> Box<dyn TaskBuilder>,
}
