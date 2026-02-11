#![cfg(feature = "registry")]

use puniyu_adapter::registry::AdapterRegistry;
use puniyu_adapter::types::info::{
	AdapterInfo, AdapterInfoBuilder, AdapterPlatform, AdapterProtocol,
};
use puniyu_version::Version;

fn create_test_adapter(name: &str) -> AdapterInfo {
	AdapterInfoBuilder::default()
		.name(name)
		.platform(AdapterPlatform::QQ)
		.protocol(AdapterProtocol::Console)
		.version(Version::new(1, 0, 0))
		.build()
		.unwrap()
}

#[test]
fn test_registry_register() {
	let adapter = create_test_adapter("test_register");
	let result = AdapterRegistry::register(adapter);
	assert!(result.is_ok());
}

#[test]
fn test_registry_get_by_index() {
	let adapter = create_test_adapter("test_get_index");
	let index = AdapterRegistry::register(adapter.clone()).unwrap();

	let result = AdapterRegistry::get_with_index(index);
	assert!(result.is_some());

	let retrieved = result.unwrap();
	assert_eq!(retrieved.name, "test_get_index");
}

#[test]
fn test_registry_get_by_name() {
	let adapter = create_test_adapter("test_get_name");
	AdapterRegistry::register(adapter).unwrap();

	let result = AdapterRegistry::get_with_adapter_name("test_get_name");
	assert!(!result.is_empty());
	assert_eq!(result[0].name, "test_get_name");
}

#[test]
fn test_registry_unregister_by_index() {
	let adapter = create_test_adapter("test_unregister_index");
	let index = AdapterRegistry::register(adapter).unwrap();

	let result = AdapterRegistry::unregister_with_index(index);
	assert!(result.is_ok());

	let retrieved = AdapterRegistry::get_with_index(index);
	assert!(retrieved.is_none());
}

#[test]
fn test_registry_unregister_by_name() {
	let adapter = create_test_adapter("test_unregister_name");
	AdapterRegistry::register(adapter).unwrap();

	let result = AdapterRegistry::unregister_with_adapter_name("test_unregister_name");
	assert!(result.is_ok());

	let retrieved = AdapterRegistry::get_with_adapter_name("test_unregister_name");
	assert!(retrieved.is_empty());
}

#[test]
fn test_registry_all() {
	let adapter1 = create_test_adapter("test_all_1");
	let adapter2 = create_test_adapter("test_all_2");

	AdapterRegistry::register(adapter1).unwrap();
	AdapterRegistry::register(adapter2).unwrap();

	let all = AdapterRegistry::all();
	assert!(all.len() >= 2);
}

#[test]
fn test_registry_unregister_nonexistent_index() {
	let result = AdapterRegistry::unregister_with_index(99999);
	assert!(result.is_err());
}

#[test]
fn test_registry_unregister_nonexistent_name() {
	let result = AdapterRegistry::unregister_with_adapter_name("nonexistent_adapter");
	assert!(result.is_err());
}

#[test]
fn test_registry_get_nonexistent_index() {
	let result = AdapterRegistry::get_with_index(99999);
	assert!(result.is_none());
}

#[test]
fn test_registry_get_nonexistent_name() {
	let result = AdapterRegistry::get_with_adapter_name("nonexistent_adapter");
	assert!(result.is_empty());
}
