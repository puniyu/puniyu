//! 命令权限定义

use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

/// 命令权限
///
/// 定义谁可以执行命令。
///
/// # 示例
///
/// ```rust
/// use puniyu_command_core::Permission;
///
/// // 所有人都可以执行
/// let all = Permission::All;
///
/// // 仅主人可以执行
/// let master = Permission::Master;
/// ```
#[derive(
	Debug, Default, Clone, PartialEq, EnumString, Display, IntoStaticStr, Deserialize, Serialize,
)]
pub enum Permission {
	/// 所有人都可以执行
	///
	/// 这是默认权限级别。
	#[strum(serialize = "all")]
	#[default]
	All,
	/// 仅主人可以执行
	///
	/// 只有配置为主人的用户才能执行此命令。
	#[strum(serialize = "master")]
	Master,
}
