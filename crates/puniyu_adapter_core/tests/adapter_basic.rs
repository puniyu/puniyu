use puniyu_adapter_core::api::AdapterApi;
use puniyu_adapter_core::types::info::AdapterInfo;

// 测试用的简单适配器实现
struct TestAdapter {
	info: AdapterInfo,
	api: AdapterApi,
}

impl TestAdapter {
	fn new(info: AdapterInfo, api: AdapterApi) -> Self {
		Self { info, api }
	}

	fn info(&self) -> &AdapterInfo {
		&self.info
	}

	fn api(&self) -> &AdapterApi {
		&self.api
	}
}

#[test]
fn test_adapter_creation() {
	let adapter = TestAdapter::new(AdapterInfo::default(), AdapterApi::default());

	let _info = adapter.info();
	let _api = adapter.api();
}

#[test]
fn test_adapter_info_access() {
	use puniyu_adapter_core::adapter_info;
	use puniyu_adapter_core::types::info::{AdapterPlatform, AdapterProtocol};
	use puniyu_version::Version;

	let info = adapter_info!(
		name: "test_adapter",
		platform: AdapterPlatform::QQ,
		protocol: AdapterProtocol::Console,
		version: Version::new(1, 0, 0),
	);

	let adapter = TestAdapter::new(info.clone(), AdapterApi::default());

	let adapter_info = adapter.info();
	assert_eq!(adapter_info.name, "test_adapter");
	assert_eq!(adapter_info.platform, AdapterPlatform::QQ);
	assert_eq!(adapter_info.protocol, AdapterProtocol::Console);
}

#[test]
fn test_adapter_api_access() {
	let adapter = TestAdapter::new(AdapterInfo::default(), AdapterApi::default());

	let api = adapter.api();
	let _message_api = api.message();
	let _group_api = api.group();
	let _friend_api = api.friend();
	let _account_api = api.account();
}

#[test]
fn test_adapter_with_custom_info() {
	use puniyu_adapter_core::adapter_info;
	use puniyu_adapter_core::types::info::{
		AdapterCommunication, AdapterPlatform, AdapterProtocol, AdapterStandard,
	};
	use puniyu_version::Version;

	let info = adapter_info!(
		name: "custom_adapter",
		platform: AdapterPlatform::QQ,
		protocol: AdapterProtocol::NapCat,
		standard: AdapterStandard::OneBotV11,
		communication: AdapterCommunication::WebSocketServer,
		version: Version::new(2, 0, 0),
	);

	let adapter = TestAdapter::new(info, AdapterApi::default());
	let adapter_info = adapter.info();

	assert_eq!(adapter_info.name, "custom_adapter");
	assert_eq!(adapter_info.platform, AdapterPlatform::QQ);
	assert_eq!(adapter_info.protocol, AdapterProtocol::NapCat);
	assert_eq!(adapter_info.standard, AdapterStandard::OneBotV11);
	assert_eq!(adapter_info.communication, AdapterCommunication::WebSocketServer);
	assert_eq!(adapter_info.version, Version::new(2, 0, 0));
}

#[test]
fn test_adapter_api_components() {
	let adapter = TestAdapter::new(AdapterInfo::default(), AdapterApi::default());
	let api = adapter.api();

	// 测试所有 API 组件都可以访问
	let _message_api = api.message();
	let _group_api = api.group();
	let _friend_api = api.friend();
	let _account_api = api.account();
}

#[test]
fn test_adapter_clone_api() {
	let api1 = AdapterApi::default();
	let api2 = api1.clone();

	let adapter1 = TestAdapter::new(AdapterInfo::default(), api1);
	let adapter2 = TestAdapter::new(AdapterInfo::default(), api2);

	// 两个适配器都应该可以正常使用
	let _msg_api1 = adapter1.api().message();
	let _msg_api2 = adapter2.api().message();
}

#[test]
fn test_adapter_multiple_instances() {
	use puniyu_adapter_core::adapter_info;
	use puniyu_adapter_core::types::info::{AdapterPlatform, AdapterProtocol};

	let info1 = adapter_info!("adapter1", AdapterPlatform::QQ, AdapterProtocol::NapCat);
	let info2 = adapter_info!("adapter2", AdapterPlatform::Telegram, AdapterProtocol::Console);

	let adapter1 = TestAdapter::new(info1, AdapterApi::default());
	let adapter2 = TestAdapter::new(info2, AdapterApi::default());

	assert_eq!(adapter1.info().name, "adapter1");
	assert_eq!(adapter2.info().name, "adapter2");
	assert_ne!(adapter1.info().platform, adapter2.info().platform);
}
