use std::sync::Arc;
use actix_web::web::ServiceConfig;

pub type ServerFunction = Arc<dyn Fn(&mut ServiceConfig) + Send + Sync>;

/// 服务器控制命令
#[derive(Debug, Clone)]
pub enum ServerCommand {
    /// 启动服务器
    Start,
    /// 停止服务器
    Stop,
    /// 重启服务器
    Restart,
}