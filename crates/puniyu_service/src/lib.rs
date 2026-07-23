use async_trait::async_trait;
use puniyu_context::ServiceContext;
use puniyu_error::AnyError;
use semver::{Comparator, Op, Version, VersionReq};

#[async_trait]
pub trait Service: Send + Sync {
	/// Service 名称
	fn name(&self) -> &str;
	/// Service 版本
	fn version(&self) -> Version;
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
	/// Service 描述
	fn description(&self) -> Option<&str> {
		None
	}
	/// Service 作者
	fn author(&self) -> Vec<&str> {
		vec![]
	}

	/// 创建并注入能力
	async fn setup(&self, _ctx: &ServiceContext) -> AnyError {
		Ok(())
	}

	/// 清理资源
	async fn cleanup(&self, _ctx: &ServiceContext) -> AnyError {
		Ok(())
	}
}

impl PartialEq for dyn Service {
	fn eq(&self, other: &Self) -> bool {
		self.name() == other.name()
	}
}

impl Eq for dyn Service {}
