use smol_str::SmolStr;

/// 提取失败的错误信息。
#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
	/// 缺少必要数据。
	#[error("missing: {0}")]
	Missing(SmolStr),
	/// 类型不匹配。
	#[error("type mismatch: expected {expected}, got {actual}")]
	TypeMismatch {
		/// 期望的类型名。
		expected: SmolStr,
		/// 实际遇到的类型。
		actual: SmolStr,
	},
}
