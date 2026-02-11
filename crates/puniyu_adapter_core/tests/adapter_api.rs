use puniyu_adapter_core::api::AdapterApi;

#[test]
fn test_adapter_api_default() {
	let api = AdapterApi::default();

	// 测试所有 API 都可以访问
	let _message_api = api.message();
	let _group_api = api.group();
	let _friend_api = api.friend();
	let _account_api = api.account();
}

#[test]
fn test_adapter_api_clone() {
	let api1 = AdapterApi::default();
	let api2 = api1.clone();

	// 克隆后的 API 应该可以正常使用
	let _message_api1 = api1.message();
	let _message_api2 = api2.message();
}

#[test]
fn test_adapter_api_message_access() {
	let api = AdapterApi::default();
	let message_api = api.message();

	// 验证可以获取消息 API
	assert_ne!(std::ptr::addr_of!(*message_api) as usize, 0);
}

#[test]
fn test_adapter_api_group_access() {
	let api = AdapterApi::default();
	let group_api = api.group();

	// 验证可以获取群组 API
	assert_ne!(std::ptr::addr_of!(*group_api) as usize, 0);
}

#[test]
fn test_adapter_api_friend_access() {
	let api = AdapterApi::default();
	let friend_api = api.friend();

	// 验证可以获取好友 API
	assert_ne!(std::ptr::addr_of!(*friend_api) as usize, 0);
}

#[test]
fn test_adapter_api_account_access() {
	let api = AdapterApi::default();
	let account_api = api.account();

	// 验证可以获取账户 API
	assert_ne!(std::ptr::addr_of!(*account_api) as usize, 0);
}

#[test]
fn test_adapter_api_multiple_access() {
	let api = AdapterApi::default();

	// 多次访问同一个 API
	let message_api1 = api.message();
	let message_api2 = api.message();

	// 应该返回相同的引用
	assert!(std::ptr::eq(std::ptr::addr_of!(**message_api1), std::ptr::addr_of!(**message_api2)));
}
