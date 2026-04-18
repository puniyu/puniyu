mod adapter;
pub use adapter::ConsoleAdapterRuntime;

use puniyu_adapter::runtime::AdapterRuntime;
use std::sync::Arc;

pub(crate) fn runtime() -> Arc<dyn AdapterRuntime> {
	Arc::new(ConsoleAdapterRuntime::new())
}
