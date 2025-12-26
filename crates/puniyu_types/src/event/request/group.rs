use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupApplyType {
	/// 申请理由
	pub(crate) reason: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupInviteType {
	pub(crate) target_id: String,
}
