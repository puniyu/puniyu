use super::utils::get_repo;
use git2::{BranchType, Error};
use std::path::Path;
use tokio::task;

pub struct BranchListInfo {
    /// 默认分支
    pub default_branch: Option<String>,
    /// 分支列表
    pub list: Vec<BranchInfo>,
}

pub struct BranchInfo {
    /// 分支索引
    pub index: usize,
    /// 分支名称
    pub name: String,
    /// 分支短hash
    pub short_hash: String,
    /// 分支长hash
    pub long_hash: String,
}

/// 获取本地分支列表
///
/// # 参数
///
/// * `path` - 仓库路径
///
/// # 返回值
///
/// * `BranchListInfo` - 分支列表信息
///
/// # 错误
///
/// * `Error` - 错误信息
pub async fn get_branch_list(path: &Path) -> Result<BranchListInfo, Error> {
    let repo = get_repo(path).await?;
    let path_buf = path.to_path_buf();

    task::spawn_blocking(move || {
        let mut branch_list = Vec::new();
        let mut index = 0;

        for branch_result in repo.branches(Some(BranchType::Local))? {
            let (branch, _) = branch_result?;
            if let Some(name) = branch.name()? {
                let commit = branch.get().peel_to_commit()?;
                let short_hash = commit.id().to_string()[..7].to_string();
                let long_hash = commit.id().to_string();

                branch_list.push(BranchInfo {
                    index,
                    name: name.to_string(),
                    short_hash,
                    long_hash,
                });

                index += 1;
            }
        }

        let default_branch = if let Ok(default_repo) = git2::Repository::open(path_buf) {
            default_repo
                .head()
                .ok()
                .and_then(|head| head.shorthand().map(|name| name.to_string()))
        } else {
            None
        };

        Ok(BranchListInfo {
            default_branch,
            list: branch_list,
        })
    })
    .await
    .unwrap()
}

/// 获取本地仓库的默认分支
///
/// # 参数
///
/// * `path` - 仓库路径
///
/// # 返回值
///
/// * `Option<String>` - 默认分支名称
///
/// # 错误
///
/// * `Error` - 错误信息
pub async fn get_local_default_branch(path: &Path) -> Result<Option<String>, Error> {
    let branch_info = get_branch_list(path).await?;
    Ok(branch_info.default_branch)
}
