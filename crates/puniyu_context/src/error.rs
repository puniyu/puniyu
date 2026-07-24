use smol_str::SmolStr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	/// 能力已存在。
	#[error("capability '{capability}' conflicts")]
	Conflict { capability: &'static str },

	/// 插件所需的能力不存在。
	#[error("capability '{capability}' is missing for '{requester}'")]
	Missing { requester: SmolStr, capability: &'static str },
}
