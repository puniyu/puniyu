//! Puniyu 构建工具包
//!
//! 这个包提供了用于构建 Puniyu 应用相关的构建脚本，如plugin构建
//!

mod plugin;

use cargo_metadata::MetadataCommand;
use serde_json::Value;
use std::env;

#[allow(clippy::collapsible_if)]
pub fn setup_plugin() {
	let metadata = MetadataCommand::new().no_deps().exec().unwrap();
	let packages = metadata.packages;
	let name = env::var("CARGO_PKG_NAME").expect("CARGO_PKG_NAME is not set");
	let package = packages.iter().find(|p| p.name == name).unwrap();
	let metadata = &package.metadata;
	if let Some(puniyu_metadata) = metadata.get("puniyu") {
		if let Some(plugin) = puniyu_metadata.get("plugin") {
			match plugin {
				Value::Bool(v) => {
					if *v {
						let features = &package.features;
						plugin::check_features(features)
					}
				}
				Value::String(v) => {
					if v == "true" {
						let features = &package.features;
						plugin::check_features(features)
					}
				}
				_ => {
					panic!("plugin must be a bool or string")
				}
			}
		} else {
			panic!("Unknown plugin type encountered in metadata");
		}
	} else {
		panic!("Unknown plugin type encountered in metadata");
	}
}
