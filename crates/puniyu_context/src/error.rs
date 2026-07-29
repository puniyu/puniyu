use smol_str::SmolStr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	/// 服务名冲突
	#[error("service '{name}' already exists")]
	Conflict { name: SmolStr },

	/// 服务不存在。
	#[error("service '{name}' is missing")]
	Missing { name: SmolStr },
}
