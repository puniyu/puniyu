use puniyu_core::VERSION;
use puniyu_registry::plugin;

#[plugin]
pub async fn min() {
	let version = VERSION;
	log::info!("version: {}", version);
}

mod task;
