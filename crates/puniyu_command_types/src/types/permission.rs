//! 命令权限类型。

use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

/// 命令权限级别。
#[derive(
	Debug, Default, Clone, PartialEq, EnumString, Display, IntoStaticStr, Deserialize, Serialize,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Permission {
	/// 所有人都可以执行。
	#[default]
	All,
	/// 仅主人可以执行。
	Master,
}
