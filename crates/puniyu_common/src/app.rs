use std::sync::OnceLock;

/// 当前程序名称, 也就是此实例的名称
///
/// ## 示例
/// ```rust
/// use puniyu_common::APP_NAME;
/// let name = APP_NAME.get_or_init(|| String::from("puniyu"));
/// ```
pub static APP_NAME: OnceLock<String> = OnceLock::new();
