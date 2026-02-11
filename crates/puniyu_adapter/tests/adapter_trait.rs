use puniyu_adapter::Adapter;
use puniyu_adapter::api::AdapterApi;
use puniyu_adapter::types::info::AdapterInfo;

// 测试用的简单适配器实现
struct TestAdapter {
	info: AdapterInfo,
	api: AdapterApi,
}

#[async_trait::async_trait]
impl Adapter for TestAdapter {
	fn info(&self) -> &AdapterInfo {
		&self.info
	}

	fn api(&self) -> &AdapterApi {
		&self.api
	}
}

#[test]
fn test_adapter_creation() {
	let adapter = TestAdapter { info: AdapterInfo::default(), api: AdapterApi::default() };

	let _info = adapter.info();
	let _api = adapter.api();
}

#[test]
fn test_adapter_default_config() {
	let adapter = TestAdapter { info: AdapterInfo::default(), api: AdapterApi::default() };

	let configs = adapter.config();
	assert_eq!(configs.len(), 0);
}

#[test]
fn test_adapter_default_hooks() {
	let adapter = TestAdapter { info: AdapterInfo::default(), api: AdapterApi::default() };

	let hooks = adapter.hooks();
	assert_eq!(hooks.len(), 0);
}

#[test]
fn test_adapter_default_server() {
	let adapter = TestAdapter { info: AdapterInfo::default(), api: AdapterApi::default() };

	let server = adapter.server();
	assert!(server.is_none());
}

#[tokio::test]
async fn test_adapter_init() {
	let adapter = TestAdapter { info: AdapterInfo::default(), api: AdapterApi::default() };

	let result = adapter.init().await;
	assert!(result.is_ok());
}

#[test]
fn test_adapter_info_access() {
	use puniyu_adapter::types::info::{AdapterInfoBuilder, AdapterPlatform, AdapterProtocol};
	use puniyu_version::Version;

	let info = AdapterInfoBuilder::default()
		.name("test_adapter")
		.platform(AdapterPlatform::QQ)
		.protocol(AdapterProtocol::Console)
		.version(Version::new(1, 0, 0))
		.build()
		.unwrap();

	let adapter = TestAdapter { info: info.clone(), api: AdapterApi::default() };

	let adapter_info = adapter.info();
	assert_eq!(adapter_info.name, "test_adapter");
	assert_eq!(adapter_info.platform, AdapterPlatform::QQ);
}

#[test]
fn test_adapter_api_access() {
	let adapter = TestAdapter { info: AdapterInfo::default(), api: AdapterApi::default() };

	let api = adapter.api();
	let _message_api = api.message();
	let _group_api = api.group();
	let _friend_api = api.friend();
	let _account_api = api.account();
}
#[tokio::test]
async fn test_adapter_custom_init() {
	struct CustomAdapter {
		info: AdapterInfo,
		api: AdapterApi,
		initialized: std::sync::Arc<std::sync::atomic::AtomicBool>,
	}

	#[async_trait::async_trait]
	impl Adapter for CustomAdapter {
		fn info(&self) -> &AdapterInfo {
			&self.info
		}

		fn api(&self) -> &AdapterApi {
			&self.api
		}

		async fn init(&self) -> puniyu_error::Result {
			self.initialized.store(true, std::sync::atomic::Ordering::SeqCst);
			Ok(())
		}
	}

	let initialized = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
	let adapter = CustomAdapter {
		info: AdapterInfo::default(),
		api: AdapterApi::default(),
		initialized: initialized.clone(),
	};

	assert!(!initialized.load(std::sync::atomic::Ordering::SeqCst));

	adapter.init().await.unwrap();

	assert!(initialized.load(std::sync::atomic::Ordering::SeqCst));
}
