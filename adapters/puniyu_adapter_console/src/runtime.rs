mod adapter;
pub use adapter::Runtime;

use puniyu_adapter::runtime::AdapterRuntime;
use std::sync::Arc;

pub(crate) fn runtime() -> Arc<dyn AdapterRuntime> {
	Arc::new(Runtime::new())
}
