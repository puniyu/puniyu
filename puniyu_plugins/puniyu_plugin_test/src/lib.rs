use puniyu_core::plugin::prelude::*;
use puniyu_core::{VERSION, logger};

#[plugin]
async fn min() -> Result<(), Box<dyn std::error::Error>> {
	let version = VERSION;
	logger::info!("version: {}", version);
	Ok(())
}

mod task;
