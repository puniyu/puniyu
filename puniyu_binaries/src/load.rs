pub async fn load() {
	crate::log::start_log();
	crate::log::init();
	tokio::fs::write(puniyu_path::resource_dir().join("logo.png"), crate::ASSETS).await.unwrap();
}
