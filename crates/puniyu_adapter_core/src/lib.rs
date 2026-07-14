mod error;
#[doc(inline)]
pub use error::Error;
mod registry;
#[doc(inline)]
pub use registry::AdapterRegistry;
mod types;
#[doc(inline)]
pub use types::*;

use puniyu_adapter_api::AdapterApi;
use puniyu_config::Config;
use puniyu_error::AnyError;
use salvo::Router;
use semver::{Comparator, Op, Version, VersionReq};
use std::sync::Arc;

#[async_trait::async_trait]
pub trait Adapter: Send + Sync + AdapterApi {
	/// 所需核心版本范围
	fn version_range(&self) -> VersionReq {
		const VERSION: Version = puniyu_version::VERSION;
		VersionReq {
			comparators: vec![Comparator {
				op: Op::GreaterEq,
				major: VERSION.major,
				minor: Some(VERSION.minor),
				patch: Some(VERSION.patch),
				pre: VERSION.pre,
			}],
		}
	}

	fn config(&self) -> Vec<Arc<dyn Config>> {
		Vec::new()
	}

	fn server(&self) -> Option<Router> {
		None
	}

	async fn on_load(&self) -> AnyError {
		Ok(())
	}

	async fn on_unload(&self) -> AnyError {
		Ok(())
	}
}
