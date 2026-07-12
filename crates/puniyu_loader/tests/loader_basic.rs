use async_trait::async_trait;
use std::sync::Arc;

use puniyu_loader::Loader;

struct TestLoader;

#[async_trait]
impl Loader for TestLoader {
	fn name(&self) -> &str {
		"test_loader"
	}
}

#[tokio::test]
async fn test_loader_name() {
	let loader = TestLoader;
	assert_eq!(loader.name(), "test_loader");
}

#[tokio::test]
async fn test_loader_default_adapters_empty() {
	let loader = TestLoader;
	let result = loader.adapters().await;
	assert!(result.is_ok());
	assert!(result.unwrap().is_empty());
}

#[tokio::test]
async fn test_loader_default_plugins_empty() {
	let loader = TestLoader;
	let result = loader.plugins().await;
	assert!(result.is_ok());
	assert!(result.unwrap().is_empty());
}

#[tokio::test]
async fn test_loader_trait_object() {
	let loader: Arc<dyn Loader> = Arc::new(TestLoader);
	assert_eq!(loader.name(), "test_loader");
	assert!(loader.adapters().await.unwrap().is_empty());
	assert!(loader.plugins().await.unwrap().is_empty());
}
