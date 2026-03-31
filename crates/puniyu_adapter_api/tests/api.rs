use puniyu_adapter_api::{
	AccountApi, AdapterApi, AdapterApiBuilder, FriendApi, GroupApi, MessageApi,
};
use std::sync::Arc;

struct TestGroupApi;
impl GroupApi for TestGroupApi {}

struct TestFriendApi;
impl FriendApi for TestFriendApi {}

struct TestAccountApi;
impl AccountApi for TestAccountApi {}

struct TestMessageApi;
impl MessageApi for TestMessageApi {}

#[test]
fn adapter_api_new_uses_given_sub_apis() {
	let group: Arc<dyn GroupApi> = Arc::new(TestGroupApi);
	let friend: Arc<dyn FriendApi> = Arc::new(TestFriendApi);
	let account: Arc<dyn AccountApi> = Arc::new(TestAccountApi);
	let message: Arc<dyn MessageApi> = Arc::new(TestMessageApi);

	let api = AdapterApi::new(group.clone(), friend.clone(), account.clone(), message.clone());

	assert!(Arc::ptr_eq(api.group(), &group));
	assert!(Arc::ptr_eq(api.friend(), &friend));
	assert!(Arc::ptr_eq(api.account(), &account));
	assert!(Arc::ptr_eq(api.message(), &message));
}

#[test]
fn adapter_api_builder_can_override_sub_api() {
	let message: Arc<dyn MessageApi> = Arc::new(TestMessageApi);

	let api = AdapterApiBuilder::default()
		.message_api(message.clone())
		.build()
		.expect("failed to build adapter api");

	assert!(Arc::ptr_eq(api.message(), &message));
}

#[test]
fn adapter_api_default_exposes_all_sub_apis() {
	let api = AdapterApi::default();

	let _ = api.group();
	let _ = api.friend();
	let _ = api.account();
	let _ = api.message();
}
