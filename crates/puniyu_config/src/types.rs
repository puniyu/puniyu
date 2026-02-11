use std::path::PathBuf;
use toml::Value;

pub trait Config: Send + Sync + 'static {
    /// 配置文件名称
    fn name(&self) -> &'static str;

    /// 配置项
    fn config(&self) -> Value;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ConfigInfo {
    pub path: PathBuf,
    pub value: Value,
}


#[derive(Debug, Clone, PartialEq)]
pub enum ConfigId {
    Index(u64),
    Path(PathBuf)
}

impl From<u64> for ConfigId {
    fn from(index: u64) -> Self {
        Self::Index(index)
    }
}

impl From<PathBuf> for ConfigId {
    fn from(path: PathBuf) -> Self {
        Self::Path(path)
    }
}