use puniyu_registry::plugin;

#[plugin]
pub async fn min() {
	log::info!("min");
}

mod task;
