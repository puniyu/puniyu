use puniyu_adapter::{Adapter};
use puniyu_error::registry::Error;
use std::sync::Arc;
use puniyu_common::source::SourceType;

pub async fn init_adapter(adapter: Arc<dyn Adapter>) -> Result<(), Error> {
    use puniyu_adapter::AdapterRegistry;
    let info = adapter.info();
    let index = AdapterRegistry::register(info)?;
    let source = SourceType::Adapter( index);
    super::hook::init_hook(source, adapter.hooks())?;
    #[cfg(feature = "server")]
    {
        if let Some(server) = adapter.server() {
            super::server::init_server(source, server)?;
        }
    }
    Ok(())
}
