use bytes::Bytes;
use puniyu_account::AccountInfo;
use puniyu_adapter_api::{AdapterApi, CallApi, SendMessage};
use puniyu_adapter_core::{Adapter, AdapterRegistry};
use puniyu_adapter_types::{AdapterInfo, SendMessageOptions, SendMsgResult};
use puniyu_contact::ContactType;
use puniyu_message::Message;
use std::sync::Arc;
use std::time::Duration;

struct MockAdapter {
	info: AdapterInfo,
	account: AccountInfo,
}

impl MockAdapter {
	fn new(name: &str) -> Self {
		Self {
			info: AdapterInfo::builder().name(name).build(),
			account: AccountInfo::builder().id("0").name("test").avatar(Bytes::new()).build(),
		}
	}
}

#[async_trait::async_trait]
impl SendMessage for MockAdapter {
	async fn send_message(
		&self,
		_contact: &ContactType,
		_message: &Message,
		_options: Option<&SendMessageOptions>,
	) -> puniyu_error::AnyError<SendMsgResult> {
		Ok(SendMsgResult { message_id: "0".into(), time: Duration::from_secs(0) })
	}
}

#[async_trait::async_trait]
impl CallApi for MockAdapter {
	async fn call_api(
		&self,
		_action: &str,
		_params: serde_json::Value,
	) -> puniyu_error::AnyError<serde_json::Value> {
		unimplemented!("mock")
	}
}

#[async_trait::async_trait]
impl AdapterApi for MockAdapter {
	fn adapter_info(&self) -> AdapterInfo {
		self.info.clone()
	}

	fn account_info(&self) -> AccountInfo {
		self.account.clone()
	}
}

impl Adapter for MockAdapter {}

fn make_adapter(name: &str) -> Arc<dyn Adapter> {
	Arc::new(MockAdapter::new(name))
}

#[test]
fn adapter_registry_full_lifecycle() {
	let adapter = make_adapter("integration_test");
	let index = AdapterRegistry::register(adapter).expect("register should succeed");

	let dup = make_adapter("integration_test");
	let result = AdapterRegistry::register(dup);
	assert!(result.is_err(), "duplicate name should fail");

	let found = AdapterRegistry::get(index);
	assert!(found.is_some(), "get by index should find adapter");
	assert_eq!(found.unwrap().adapter_info().name, "integration_test");

	let found = AdapterRegistry::get("integration_test".to_string());
	assert!(found.is_some(), "get by name should find adapter");
	assert_eq!(found.unwrap().adapter_info().name, "integration_test");

	let not_found = AdapterRegistry::get("nonexistent".to_string());
	assert!(not_found.is_none(), "nonexistent should return None");

	let all = AdapterRegistry::all();
	let names: Vec<String> =
		all.iter().map(|a| a.adapter_info().name.clone().to_string()).collect();
	assert!(names.contains(&"integration_test".to_string()), "all() should contain registered");

	let unreg_result = AdapterRegistry::unregister("integration_test".to_string());
	assert!(unreg_result.is_ok(), "unregister by name should succeed");

	let after_unreg = AdapterRegistry::get("integration_test".to_string());
	assert!(after_unreg.is_none(), "should be gone after unregister");

	let err = AdapterRegistry::unregister("nonexistent".to_string());
	assert!(err.is_err(), "unregister nonexistent should error");
}

#[test]
fn adapter_registry_register_and_unregister_by_index() {
	let adapter = make_adapter("index_test");
	let index = AdapterRegistry::register(adapter).expect("register should succeed");

	assert!(AdapterRegistry::get(index).is_some());

	let result = AdapterRegistry::unregister(index);
	assert!(result.is_ok(), "unregister by index should succeed");

	assert!(AdapterRegistry::get(index).is_none());
}

#[test]
fn adapter_registry_unregister_nonexistent_index() {
	let result = AdapterRegistry::unregister(999999);
	assert!(result.is_err(), "unregister nonexistent index should error");
}

#[test]
fn adapter_registry_get_by_nonexistent_index() {
	let result = AdapterRegistry::get(999999);
	assert!(result.is_none(), "get nonexistent index should return None");
}

#[test]
fn adapter_registry_all_contains_only_registered() {
	let adapter = make_adapter("all_test_unique");
	AdapterRegistry::register(adapter).expect("register should succeed");

	let all = AdapterRegistry::all();
	let names: Vec<String> =
		all.iter().map(|a| a.adapter_info().name.clone().to_string()).collect();
	assert!(names.contains(&"all_test_unique".to_string()), "all() should contain registered");

	AdapterRegistry::unregister("all_test_unique".to_string()).ok();
}
