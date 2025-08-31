use log::info;
use puniyu_registry::plugin;

#[plugin]
pub async fn min() {
    info!("min")
}
mod task;
