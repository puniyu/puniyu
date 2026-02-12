use std::fmt;

/// 权限枚举
///
/// 定义功能的使用权限级别，用于控制谁可以使用特定功能。
///
/// # 变体
///
/// - `All` - 所有人可用（默认）
/// - `Master` - 仅主人可用
///
/// # 示例
///
/// ## 基本使用
///
/// ```rust
/// use puniyu_event::Permission;
///
/// let perm = Permission::Master;
/// assert_eq!(perm.to_string(), "master");
///
/// let perm = Permission::All;
/// assert_eq!(perm.to_string(), "all");
/// ```
///
/// ## 从字符串解析
///
/// ```rust
/// use puniyu_event::Permission;
/// use std::str::FromStr;
///
/// let perm = Permission::from_str("master").unwrap();
/// assert_eq!(perm, Permission::Master);
///
/// let perm = Permission::from_str("all").unwrap();
/// assert_eq!(perm, Permission::All);
///
/// // 不区分大小写
/// let perm = Permission::from_str("MASTER").unwrap();
/// assert_eq!(perm, Permission::Master);
/// ```
///
/// ## 在命令系统中使用
///
/// ```rust,ignore
/// use puniyu_event::Permission;
///
/// struct Command {
///     name: String,
///     permission: Permission,
/// }
///
/// impl Command {
///     fn can_execute(&self, is_master: bool) -> bool {
///         match self.permission {
///             Permission::All => true,
///             Permission::Master => is_master,
///         }
///     }
/// }
///
/// let cmd = Command {
///     name: "admin".to_string(),
///     permission: Permission::Master,
/// };
///
/// assert!(cmd.can_execute(true));   // 主人可以执行
/// assert!(!cmd.can_execute(false)); // 非主人不能执行
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Permission {
	/// 所有人可用（默认）
	#[default]
	All,
	/// 仅主人可用
	Master,
}

impl std::str::FromStr for Permission {
	type Err = std::convert::Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s.to_lowercase().as_str() {
			"master" => Permission::Master,
			_ => Permission::All,
		})
	}
}

impl fmt::Display for Permission {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Permission::All => write!(f, "all"),
			Permission::Master => write!(f, "master"),
		}
	}
}
