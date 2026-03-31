#![cfg(feature = "registry")]

use async_trait::async_trait;
use puniyu_adapter_api::AdapterApi;
use puniyu_adapter_core::{Adapter, AdapterRegistry};
use puniyu_adapter_types::{AdapterInfo, AdapterPlatform, AdapterProtocol, adapter_info};
use std::sync::{Arc, Mutex, MutexGuard};

static TEST_LOCK: Mutex<()> = Mutex::new(());

struct TestAdapter {
	info: AdapterInfo,
	api: AdapterApi,
}

impl TestAdapter {
	fn new() -> Self {
		Self {
			info: adapter_info!("console", AdapterPlatform::QQ, AdapterProtocol::Console),
			api: AdapterApi::default(),
		}
	}
}

#[async_trait]
impl Adapter for TestAdapter {
	fn info(&self) -> AdapterInfo {
		self.info.clone()
	}

	fn api(&self) -> AdapterApi {
		self.api.clone()
	}
}

fn test_guard() -> MutexGuard<'static, ()> {
	TEST_LOCK.lock().expect("failed to acquire adapter registry test lock")
}

fn cleanup() {
	let _ = AdapterRegistry::unregister_with_adapter_name("console");
}

#[test]
fn register_returns_index_and_makes_adapter_queryable() {
	let _guard = test_guard();
	cleanup();

	let adapter: Arc<dyn Adapter> = Arc::new(TestAdapter::new());
	let index = AdapterRegistry::register(adapter).expect("failed to register adapter");

	let by_index = AdapterRegistry::get_with_index(index).expect("adapter should exist by index");
	assert_eq!(by_index.info().name, "console");

	let by_name = AdapterRegistry::get_with_adapter_name("console");
	assert_eq!(by_name.len(), 1);

	cleanup();
}

#[test]
fn duplicate_registration_returns_exists_error() {
	let _guard = test_guard();
	cleanup();

	let first: Arc<dyn Adapter> = Arc::new(TestAdapter::new());
	let second: Arc<dyn Adapter> = Arc::new(TestAdapter::new());

	AdapterRegistry::register(first).expect("first register should succeed");
	let err = AdapterRegistry::register(second).expect_err("duplicate register should fail");

	assert!(err.to_string().contains("exists"));

	cleanup();
}
