mod error;
pub use error::Error;
mod types;
#[doc(inline)]
pub use types::*;
mod registry;
pub use registry::PluginRegistry;

use async_trait::async_trait;
use puniyu_config::Config;
use puniyu_error::AnyError;
use salvo::Router;
use semver::{Comparator, Op, Version, VersionReq};
use std::sync::Arc;

#[async_trait]
pub trait Plugin: Send + Sync {
	/// 插件名称
	fn name(&self) -> &str;
	/// 插件版本
	fn version(&self) -> Version;
	/// 核心版本范围
	fn version_range(&self) -> VersionReq {
		const VERSION: Version = puniyu_version::VERSION;
		VersionReq {
			comparators: vec![Comparator {
				op: Op::GreaterEq,
				major: VERSION.major,
				minor: Some(VERSION.minor),
				patch: Some(VERSION.patch),
				pre: VERSION.pre,
			}],
		}
	}
	/// 插件描述
	fn description(&self) -> Option<&str> {
		None
	}
	/// 插件作者
	fn author(&self) -> Vec<&str> {
		vec![]
	}

	/// 任务列表
	fn tasks(&self) -> Vec<Arc<dyn puniyu_task::Task>> {
		Vec::new()
	}

	/// 命令列表
	fn commands(&self) -> Vec<Arc<dyn puniyu_command::Command>> {
		Vec::new()
	}

	/// 插件配置文件
	fn config(&self) -> Vec<Arc<dyn Config>> {
		Vec::new()
	}

	fn server(&self) -> Option<Router> {
		None
	}

	/// 插件加载时回调
	async fn on_load(&self) -> AnyError {
		Ok(())
	}

	/// 插件卸载时回调
	async fn on_unload(&self) -> AnyError {
		Ok(())
	}
}

impl PartialEq for dyn Plugin {
	fn eq(&self, other: &Self) -> bool {
		self.name() == other.name()
			&& self.tasks() == other.tasks()
			&& self.commands() == other.commands()
	}
}
