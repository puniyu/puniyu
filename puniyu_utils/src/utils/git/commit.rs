use super::utils::get_repo;
use git2::Error;
use std::path::Path;
use tokio::task;

pub struct CommitInfo {
    /// 提交hash
    pub hash: String,
    /// 提交短hash
    pub short_hash: String,
    /// 提交信息
    pub message: String,
}

/// 获取本地提交信息
///
/// # 参数
///
/// * `path` - 仓库路径
///
/// # 返回值
///
/// * `CommitInfo` - 提交信息
///
/// # 错误
///
/// * `Error` - 错误信息
pub async fn get_local_commit_info(path: &Path) -> Result<CommitInfo, Error> {
    let repo = get_repo(path).await?;
    task::spawn_blocking(move || {
        let head = repo.head()?;
        let commit = head.peel_to_commit()?;
        Ok(create_commit_info(&commit))
    })
    .await
    .unwrap()
}

/// 获取本地提交hash
///
/// # 参数
///
/// * `path` - 仓库路径
///
/// # 返回值
///
/// * `String` - 提交hash
///
/// # 错误
///
/// * `Error` - 错误信息
pub async fn get_local_commit_hash(path: &Path) -> Result<String, Error> {
    let commit_info = get_local_commit_info(path).await?;
    Ok(commit_info.hash)
}

/// 获取远程提交hash
///
/// # 参数
///
/// * `path` - 仓库路径
///
/// # 返回值
///
/// * `String` - 提交hash
///
/// # 错误
///
/// * `Error` - 错误信息
pub async fn get_remote_commit_hash(path: &Path) -> Result<String, Error> {
    let commit_info = get_remote_commit_info(path).await?;
    Ok(commit_info.hash)
}

/// 获取远程提交信息
///
/// # 参数
///
/// * `path` - 仓库路径
///
/// # 返回值
///
/// * `CommitInfo` - 提交信息
///
/// # 错误
///
/// * `Error` - 错误信息
pub async fn get_remote_commit_info(path: &Path) -> Result<CommitInfo, Error> {
    let repo = get_repo(path).await?;
    task::spawn_blocking(move || {
        let mut remote = repo.find_remote("origin")?;
        let mut fetch_options = git2::FetchOptions::new();
        remote.fetch(
            &["+HEAD:refs/remotes/origin/HEAD"],
            Some(&mut fetch_options),
            None,
        )?;
        let head = repo.head()?;
        let commit = head.peel_to_commit()?;
        Ok(create_commit_info(&commit))
    })
    .await
    .unwrap()
}

fn create_commit_info(commit: &git2::Commit) -> CommitInfo {
    CommitInfo {
        hash: commit.id().to_string(),
        short_hash: commit.id().to_string()[..7].to_string(),
        message: commit.message().unwrap().to_string(),
    }
}
