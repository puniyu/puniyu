//! # puniyu_app
//!
//! 应用上下文管理库，提供全局 App 实例。

use semver::Version;
use std::path::PathBuf;
use std::sync::OnceLock;

struct Inner {
	/// 应用名称
	name: &'static str,
	/// 工作目录
	cwd_dir: &'static PathBuf,
	/// 应用版本
	version: Version,
}

static APP: OnceLock<Inner> = OnceLock::new();

pub struct App;

impl App {
	/// 初始化全局 App 实例
	///
	/// 只会在首次调用时生效，后续调用会被忽略。
	pub fn init(name: &'static str, cwd_dir: &'static PathBuf, version: Version) {
		APP.get_or_init(|| Inner { name, cwd_dir, version });
	}

	/// 获取应用名称
	pub fn name() -> &'static str {
		APP.get().expect("App not initialized").name
	}

	/// 获取工作目录
	pub fn cwd_dir() -> &'static PathBuf {
		APP.get().expect("App not initialized").cwd_dir
	}

	/// 获取应用版本
	pub fn version() -> &'static Version {
		&APP.get().expect("App not initialized").version
	}
}
