use std::sync::Arc;
use puniyu_logger::error;
use puniyu_common::source::SourceType;
use puniyu_error::registry::Error;
use puniyu_hook::Hook;

pub fn init_hook(source: SourceType, hooks: Vec<Arc<dyn Hook>>) -> Result<(), Error>{
    use puniyu_hook::HookRegistry;
    for hook in hooks {
        if let Err(e) = HookRegistry::register(source, hook) {
            error!("Failed to register hook: {:?}", e);
        }
    }
    Ok(())
}