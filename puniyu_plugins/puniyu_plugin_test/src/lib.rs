use log::info;
use puniyu_registry::plugin;

#[plugin]
pub async fn min() {
    info!("插件开始执行，准备发送消息");
    info!("消息已发送");
}
mod task;
