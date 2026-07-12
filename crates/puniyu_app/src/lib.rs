//! # puniyu_app
//!
//! 应用上下文管理库，提供全局 App 实例。

use std::path::PathBuf;
use std::sync::OnceLock;

struct Inner {
	/// 应用名称
	name: &'static str,
	/// 工作目录
	cwd_dir: &'static PathBuf,
}

impl Inner {
	const fn new(name: &'static str, cwd_dir: &'static PathBuf) -> Self {
		Self { name, cwd_dir }
	}
}

static APP: OnceLock<Inner> = OnceLock::new();

pub struct App;

impl App {
	/// 初始化全局 App 实例
	///
	/// 只会在首次调用时生效，后续调用会被忽略。
	pub fn init(name: &'static str, cwd_dir: &'static PathBuf) {
		APP.get_or_init(|| Inner::new(name, cwd_dir));
	}

	/// 获取应用名称
	pub fn name() -> &'static str {
		APP.get().expect("App not initialized").name
	}

	/// 获取工作目录
	pub fn cwd_dir() -> &'static PathBuf {
		APP.get().expect("App not initialized").cwd_dir
	}
}
