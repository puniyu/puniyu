use puniyu_core::Version;
pub use puniyu_core::adapter::types::*;
pub use puniyu_core::adapter::{AdapterApi, AdapterId, AdapterRegistry};
pub use puniyu_core::adapter::Adapter;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct AdapterInfo {
	pub name: SmolStr,
	pub author: Vec<SmolStr>,
	pub description: Option<SmolStr>,
	pub version: Version,
}

pub fn get_adapter<'a>(adapter: impl Into<AdapterId<'a>>) -> Option<AdapterInfo> {
	let adapter_id = adapter.into();
	let adapter = match adapter_id {
		AdapterId::Index(index) => AdapterRegistry::get_with_index(index),
		AdapterId::Name(name) => AdapterRegistry::get_with_adapter_name(name.as_ref()),
	};
	adapter.map(|adapter| AdapterInfo {
		name: adapter.adapter_info().name,
		author: adapter.adapter_info().author,
		description: adapter.adapter_info().description,
		version: adapter.adapter_info().version,
	})
}
