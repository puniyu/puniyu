use puniyu_core::config::Config;

#[test]
fn app_config() {
	let config = Config::app();
	let logger = config.logger();
	assert!(!logger.level().is_empty());
	assert!(!logger.path().is_empty());
	assert!(logger.retention_days() > 0);
}
#[test]
fn bot_config() {
	let config = Config::bot();
	let bot_config = config.bot("default");
	let _cd = bot_config.cd();
	let _user_cd = bot_config.user_cd();
	assert!(!config.masters().is_empty());
}

#[test]
fn group_config() {
	let config = Config::group();
	let group_config = config.group("default");
	let _ = group_config.cd();
	let _ = group_config.user_cd();
}
