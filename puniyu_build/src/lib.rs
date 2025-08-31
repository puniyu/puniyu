//! Puniyu 构建工具包
//!
//! 这个包提供了用于构建 Puniyu 插件的通用构建脚本功能。

use std::env;

/// 执行插件构建时需要的环境变量设置
///
/// 这个函数会设置以下环境变量：
/// - `PLUGIN_NAME`: 从 `CARGO_PKG_NAME` 获取
/// - `PLUGIN_VERSION`: 从 `CARGO_PKG_VERSION` 获取
/// - `PLUGIN_AUTHOR`: 从 `CARGO_PKG_AUTHORS` 获取
/// - `PLUGIN_RUSTC_VERSION`: 使用 rustc_version 获取当前 rustc 版本
pub fn setup_plugin() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=build.rs");

    let plugin_name = env::var("CARGO_PKG_NAME").unwrap();
    println!("cargo:rustc-env=PLUGIN_NAME={}", plugin_name);

    let plugin_version = env::var("CARGO_PKG_VERSION").unwrap();
    println!("cargo:rustc-env=PLUGIN_VERSION={}", plugin_version);

    let plugin_author = env::var("CARGO_PKG_AUTHORS").unwrap();
    println!("cargo:rustc-env=PLUGIN_AUTHOR={}", plugin_author);

    let plugin_rustc_version = rustc_version::version().unwrap().to_string();
    println!(
        "cargo:rustc-env=PLUGIN_RUSTC_VERSION={}",
        plugin_rustc_version
    );
}
