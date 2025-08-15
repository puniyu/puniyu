use std::{env::current_dir, path::PathBuf, sync::LazyLock};

/// 配置文件夹路径
///
/// # 示例
///
/// ```
/// use puniyu_utils::path::CONFIG_DIR;
/// let config_dir = CONFIG_DIR.as_path();
/// ```
pub static CONFIG_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path = current_dir().unwrap();
    path.push("@puniyu/config");
    path
});
