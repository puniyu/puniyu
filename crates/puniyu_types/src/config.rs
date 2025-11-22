use serde_json::Value;

pub trait Config: Send + Sync {
    /// 配置文件名称
    fn name(&self) -> &'static str;

    /// 配置项
    fn config(&self) -> Value;
}