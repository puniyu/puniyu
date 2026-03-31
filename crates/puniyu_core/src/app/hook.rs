use puniyu_common::source::SourceType;
use puniyu_error::registry::Error;
use puniyu_hook::Hook;
use std::sync::Arc;

pub fn init_hook(source: SourceType, hooks: Vec<Arc<dyn Hook>>) -> Result<(), Error> {
	use puniyu_hook::HookRegistry;
	for hook in hooks {
		HookRegistry::register(source, hook)?;
	}
	Ok(())
}
