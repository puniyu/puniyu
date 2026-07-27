use async_trait::async_trait;
use puniyu_context::AdapterContext;
use puniyu_error::AnyError;
use semver::{Comparator, Op, Version, VersionReq};


#[async_trait]
pub trait Adapter: Send + Sync {
	/// 适配器名称
	fn name(&self) -> &str;
	/// 适配器版本
	fn version(&self) -> Version;
	/// 生命周期优先级，数值越小越先执行
	fn priority(&self) -> u32 {
		500
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


	/// 调用平台原生 API
	async fn call_api(
		&self,
		action: &str,
		params: serde_json::Value,
	) -> AnyError<serde_json::Value>;

		/// 插件启动回调。用于创建、发布并启动插件自己拥有的能力。
	async fn on_start(&self, _ctx: &AdapterContext) -> AnyError {
		Ok(())
	}

	/// 插件加载回调。用于取得其他能力并完成跨插件装配。
	async fn on_load(&self, _ctx: &AdapterContext) -> AnyError {
		Ok(())
	}

	/// 插件卸载回调。用于解除加载阶段完成的跨插件装配。
	async fn on_unload(&self, _ctx: &AdapterContext) -> AnyError {
		Ok(())
	}

	/// 插件停止回调。用于停止插件自己拥有的运行时资源。
	async fn on_stop(&self, _ctx: &AdapterContext) -> AnyError {
		Ok(())
	}
}
