use async_trait::async_trait;
use puniyu_adapter_core::Adapter;
use puniyu_context::AdapterContext;
use puniyu_error::AnyError;
use semver::{Comparator, Op, Version, VersionReq};

/// 适配器生命周期扩展 trait。
///
/// 在 [`Adapter`] 的平台能力之上，添加生命周期钩子和运行时配置。
/// 适配器实现者需要同时实现 `Adapter` 和 `AdapterPlugin`。
#[async_trait]
pub trait AdapterPlugin: Adapter {
	/// 生命周期优先级，数值越小越先执行。默认 500。
	fn priority(&self) -> u32 {
		500
	}

	/// 所需核心版本范围
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

	/// 启动连接、监听循环和其他适配器自己拥有的运行时资源。
	async fn on_start(&self, ctx: &AdapterContext) -> AnyError;

	/// 取得共享能力并完成跨组件装配。
	async fn on_load(&self, _ctx: &AdapterContext) -> AnyError {
		Ok(())
	}

	/// 解除加载阶段完成的跨组件装配。
	async fn on_unload(&self, _ctx: &AdapterContext) -> AnyError {
		Ok(())
	}

	/// 停止运行时资源并等待其退出。
	async fn on_stop(&self, _ctx: &AdapterContext) -> AnyError {
		Ok(())
	}
}
