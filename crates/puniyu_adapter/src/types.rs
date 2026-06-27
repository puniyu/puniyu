use puniyu_core::Version;
pub use puniyu_core::adapter::types::*;
pub use puniyu_core::adapter::{AdapterApi, AdapterId, AdapterRegistry};
pub use puniyu_core::adapter::Adapter;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct AdapterMetadata {
	pub name: SmolStr,
	pub author: Vec<SmolStr>,
	pub description: Option<SmolStr>,
	pub version: Version,
}

pub fn get_adapter<'a>(adapter: impl Into<AdapterId<'a>>) -> Option<AdapterMetadata> {
	let adapter_id = adapter.into();
	let adapter = match adapter_id {
		AdapterId::Index(index) => AdapterRegistry::get_with_index(index),
		AdapterId::Name(name) => AdapterRegistry::get_with_adapter_name(name.as_ref()),
	};
	adapter.map(|adapter| {
		let info = adapter.get().adapter_info();
		AdapterMetadata {
			name: info.name,
			author: info.author,
			description: info.description,
			version: info.version,
		}
	})
}
