mod command;
mod task;

use puniyu_core::{VERSION, logger};
pub use puniyu_plugin::prelude::*;

#[plugin]
async fn min() -> Result<(), Box<dyn std::error::Error>> {
	let version = VERSION;
	logger::info!("version: {}", version);
	Ok(())
}
