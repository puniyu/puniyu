use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrivateApplyType {
	/// 验证信息
	pub message: String,
}
