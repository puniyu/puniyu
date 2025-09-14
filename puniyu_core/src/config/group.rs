use crate::config::reload_config;
use crate::error::Config as Error;
use puniyu_utils::path::CONFIG_DIR;
use puniyu_utils::utils::toml;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, LazyLock, RwLock},
};

pub(crate) static GROUP_CONFIG: LazyLock<Arc<RwLock<GroupConfig>>> =
    LazyLock::new(|| Arc::new(RwLock::new(GroupConfig::get())));
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupConfigFile {
    /// 全局cd冷却时间
    #[serde(default = "default_group_cd")]
    pub cd: u8,
    /// 用户cd冷却时间
    #[serde(default = "default_group_user_cd")]
    pub user_cd: u8,
}

fn default_group_cd() -> u8 {
    0
}

fn default_group_user_cd() -> u8 {
    0
}

impl GroupConfigFile {
    pub fn cd(&self) -> u8 {
        self.cd
    }
    pub fn user_cd(&self) -> u8 {
        self.user_cd
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GroupConfig {
    #[serde(default)]
    group: HashMap<String, GroupConfigFile>,
}

impl Default for GroupConfigFile {
    fn default() -> Self {
        Self {
            cd: default_group_cd(),
            user_cd: default_group_user_cd(),
        }
    }
}

impl GroupConfig {
    /// 根据群组ID获取群组配置
    ///
    /// # 参数
    ///
    /// `group_id` - 群组ID
    ///
    /// # 返回值
    ///
    /// 返回对应群组的配置，如果找不到则返回默认配置
    pub fn get() -> Self {
        toml::read_config(CONFIG_DIR.as_path(), "group").unwrap_or_else(|_| GroupConfig::default())
    }

    /// 获取全局cd冷却时间
    ///
    /// # 参数
    ///
    /// `group_id` - 群组ID
    pub fn group(&self, group_id: &str) -> GroupConfigFile {
        let config = GROUP_CONFIG.read().unwrap();
        config
            .group
            .get(group_id)
            .cloned()
            .unwrap_or_else(GroupConfigFile::default)
    }
}
