use regex::Regex;

pub struct PluginInfo {
    /// 插件名称
    name: &'static str,
    /// 插件匹配正则
    reg: Regex,
    /// 插件优先级
    ///
    /// 数字越大，优先级越高
    ///
    /// 默认值为100
    priority: u64,
    /// 插件描述
    description: &'static str,
    /// 插件作者
    author: &'static str,
    /// 插件版本
    version: &'static str,
}
