//! Puniyu 构建工具包
//!
//! 这个包提供了用于构建 Puniyu 应用相关的构建脚本，如plugin，core构建

use std::env;

/// 执行插件构建时需要的环境变量设置
///
/// 这个函数会设置以下环境变量：
/// - `PLUGIN_NAME`: 从 `CARGO_PKG_NAME` 获取
/// - `PLUGIN_VERSION`: 从 `CARGO_PKG_VERSION` 获取
/// - `PLUGIN_AUTHOR`: 从 `CARGO_PKG_AUTHORS` 获取
pub fn setup_plugin() {
	println!("cargo:rerun-if-changed=build.rs");
	println!("cargo:rerun-if-changed=Cargo.toml");
	println!("cargo:rerun-if-changed=src/lib.rs");

	let plugin_name =
		env::var("CARGO_PKG_NAME").expect("呜哇～CARGO_PKG_NAME什么的根本找不到啦～杂鱼杂鱼～");
	println!("cargo:rustc-env=PLUGIN_NAME={}", plugin_name);

	let plugin_version =
		env::var("CARGO_PKG_VERSION").expect("版本号什么的...难道杂鱼忘记设置了吗～？");
	println!("cargo:rustc-env=PLUGIN_VERSION={}", plugin_version);

	let plugin_author =
		env::var("CARGO_PKG_AUTHORS").expect("作者信息都没有...真是个粗心的杂鱼呢～");
	println!("cargo:rustc-env=PLUGIN_AUTHOR={}", plugin_author);
}

/// 执行适配器构建时需要设置的环境变量
///
/// ADAPTER_NAME: 从 CARGO_PKG_NAME 获取适配器名称
/// ADAPTER_VERSION: 从 CARGO_PKG_VERSION 获取适配器版本
/// ADAPTER_AUTHOR: 从 CARGO_PKG_AUTHORS 获取适配器作者信息
pub fn setup_adapter() {
	println!("cargo:rerun-if-changed=build.rs");
	println!("cargo:rerun-if-changed=Cargo.toml");
	println!("cargo:rerun-if-changed=src/lib.rs");

	let plugin_name =
		env::var("CARGO_PKG_NAME").expect("呜哇～CARGO_PKG_NAME什么的根本找不到啦～杂鱼杂鱼～");
	println!("cargo:rustc-env=ADAPTER_NAME={}", plugin_name);

	let plugin_version =
		env::var("CARGO_PKG_VERSION").expect("版本号什么的...难道杂鱼忘记设置了吗～？");
	println!("cargo:rustc-env=ADAPTER_VERSION={}", plugin_version);

	let plugin_author =
		env::var("CARGO_PKG_AUTHORS").expect("作者信息都没有...真是个粗心的杂鱼呢～");
	println!("cargo:rustc-env=ADAPTER_AUTHOR={}", plugin_author);
}

/// 执行核心构建时需要设置的环境变量
///
/// CORE_VERSION: 完整版本号
/// CORE_VERSION_MAJOR: 主版本号
/// CORE_VERSION_MINOR: 次版本号
/// CORE_VERSION_PATCH: 修订版本号
/// CORE_VERSION_CHANNEL: 版本通道（根据 core_preview feature 设置为 "Preview" 或 "Stable"）
pub fn setup_core() {
	let version = env!("CARGO_PKG_VERSION");
	println!("cargo:rustc-env=CORE_VERSION={}", version);

	let version_parts: Vec<&str> = version.split(|c: char| !c.is_ascii_digit()).collect();
	let major = version_parts.first().and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);
	let minor = version_parts.get(1).and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);
	let patch = version_parts.get(2).and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);

	println!("cargo:rustc-env=CORE_VERSION_MAJOR={}", major);
	println!("cargo:rustc-env=CORE_VERSION_MINOR={}", minor);
	println!("cargo:rustc-env=CORE_VERSION_PATCH={}", patch);

	#[cfg(feature = "core_preview")]
	println!("cargo:rustc-env=CORE_VERSION_CHANNEL=Preview");

	#[cfg(not(feature = "core_preview"))]
	println!("cargo:rustc-env=CORE_VERSION_CHANNEL=Stable");
}
