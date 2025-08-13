use git2::{Error, Repository};
use std::path::Path;
use tokio::task;

/// 获取git仓库
///
/// # 参数
///
/// * `path` - 仓库路径
///
/// # 返回值
///
/// * `Repository` - git仓库
///
pub(crate) async fn get_repo(path: &Path) -> Result<Repository, Error> {
    let path_buf = path.to_path_buf();

    if !path_buf.exists() {
        log::error!(
            "[Git]路径不存在: {}",
            path_buf.to_str().unwrap_or("无效路径")
        );
        return Err(Error::from_str("路径不存在"));
    }

    task::spawn_blocking(move || {
        let repo = Repository::open(&path_buf)?;
        log::debug!(
            "[Git]成功打开仓库: {}",
            path_buf.to_str().unwrap_or("未知路径")
        );
        Ok(repo)
    })
    .await
    .unwrap()
}
