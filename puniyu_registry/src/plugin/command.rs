use regex::Regex;
use std::future::Future;
use std::pin::Pin;

/// 插件命令结构体
pub struct PluginCommand {
    /// 命令名称
    pub name: &'static str,
    /// 命令匹配正则表达式
    pub reg: Regex,
    /// 命令优先级，数值越大优先级越高
    pub priority: u64,
}

/// 命令处理函数类型
pub type CommandHandler =
    fn() -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'static>>;
