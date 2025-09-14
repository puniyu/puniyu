use log::info;
use puniyu_registry::plugin;

#[plugin]
pub async fn min() {
    info!("泥个猪咪")
}

mod task;
