use puniyu_utils::adapter::AdapterBase as AdapterBuilder;

pub struct AdapterRegistry {
    /// 适配器名称
    pub name: &'static str,
    /// 构建函数
    pub builder: fn() -> Box<dyn AdapterBuilder>,
}
