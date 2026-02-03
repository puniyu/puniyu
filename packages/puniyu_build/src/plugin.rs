use std::collections::BTreeMap;

pub fn check_features(features: &BTreeMap<String, Vec<String>>) {
	if features.is_empty() {
		eprintln!("Error: Plugin requires at least one feature to be enabled for cdylib build.");
		eprintln!("Please add [lib] crate-type = [\"cdylib\"] configuration in your Cargo.toml");
		panic!("Plugin build failed: missing required cdylib configuration");
	}
}
