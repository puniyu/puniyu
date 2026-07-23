use async_trait::async_trait;
use puniyu_context::PluginContext;
use puniyu_error::AnyError;
use semver::{Comparator, Op, Version, VersionReq};

#[async_trait]
pub trait Plugin: Send + Sync {
	/// 插件名称
	fn name(&self) -> &str;
	/// 插件版本
	fn version(&self) -> Version;
	/// 生命周期优先级，数值越小越先执行。
	fn priority(&self) -> u32 {
		500
	}
	/// 依赖项, 表示所依赖的service
	fn using(&self) -> Vec<&str> {
		vec![]
	}
	/// 核心版本范围
	fn required_version(&self) -> VersionReq {
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

	/// 插件启动回调。用于创建、发布并启动插件自己拥有的能力。
	async fn on_start(&self, _ctx: &PluginContext) -> AnyError {
		Ok(())
	}

	/// 插件加载回调。用于取得其他能力并完成跨插件装配。
	async fn on_load(&self, _ctx: &PluginContext) -> AnyError {
		Ok(())
	}

	/// 插件卸载回调。用于解除加载阶段完成的跨插件装配。
	async fn on_unload(&self, _ctx: &PluginContext) -> AnyError {
		Ok(())
	}

	/// 插件停止回调。用于停止插件自己拥有的运行时资源。
	async fn on_stop(&self, _ctx: &PluginContext) -> AnyError {
		Ok(())
	}
}

impl PartialEq for dyn Plugin {
	fn eq(&self, other: &Self) -> bool {
		self.name() == other.name()
	}
}
