use std::{env::current_dir, path::PathBuf, sync::LazyLock};

static BASE_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path = current_dir().unwrap();
    path.push("@puniyu");
    path
});

/// 配置文件夹路径
///
/// # 示例
///
/// ```
/// use puniyu_utils::path::CONFIG_DIR;
/// let config_dir = CONFIG_DIR.as_path();
/// ```
pub static CONFIG_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path = BASE_DIR.to_path_buf();
    path.push("config");
    path
});

/// 临时文件夹路径
///
/// # 示例
///
/// ```
/// use puniyu_utils::path::TEMP_DIR;
/// let temp_dir = TEMP_DIR.as_path();
/// ```
pub static TEMP_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path = BASE_DIR.clone();
    path.push("temp");
    path
});
