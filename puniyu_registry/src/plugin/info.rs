use std::pin::Pin;

pub trait PluginInfo: Send + Sync + 'static {
    /// 插件名称
    fn name(&self) -> &'static str;
    /// 插件版本
    fn version(&self) -> &'static str;

    fn rustc_version(&self) -> &'static str;

    // fn description(&self) -> &'static str;
    /// 插件作者
    fn author(&self) -> &'static str;
    /// 插件初始化函数
    fn init(&self) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>;
}
