use crate::ScopeId;
use smol_str::SmolStr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	/// 能力已被其他作用域占用。
	#[error("capability '{capability}' conflicts with scope {scope}")]
	Conflict { capability: &'static str, scope: ScopeId },

	/// 插件所需的能力不存在。
	#[error("capability '{capability}' is missing for '{requester}'")]
	Missing { requester: SmolStr, capability: &'static str },
}
