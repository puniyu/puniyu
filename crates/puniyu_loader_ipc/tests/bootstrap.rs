use std::path::{Path, PathBuf};
use std::sync::Once;

fn init_test_env() {
	use puniyu_common::app::{AppInfo, set_app_info};

	static INIT: Once = Once::new();
	static VERSION: &puniyu_version::Version = &puniyu_version::Version::new(0, 1, 0);

	INIT.call_once(|| {
		let info = AppInfo::new("testapp", VERSION, Path::new("."));
		set_app_info(info);
	});
}

#[test]
fn placeholder_bootstrap_test() {
	init_test_env();
	let plugin_dir = puniyu_path::plugin_dir();
	let expected = PathBuf::from(puniyu_common::app::app_cwd_dir()).join("plugins");
	assert_eq!(plugin_dir, expected);
}
