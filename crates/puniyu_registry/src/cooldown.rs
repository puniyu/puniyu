use crate::store::{CooldownScope, STORE};

pub struct CooldownRegistry;

impl CooldownRegistry {
    /// 检查好友消息是否处于冷却期
    pub fn is_friend_cooling_down(bot_id: &str, user_id: &str) -> bool {
        STORE.cooldown().is_cooling_down(&CooldownScope::Friend { bot_id, user_id })
    }

    /// 检查群组是否处于冷却期
    pub fn is_group_cooling_down(bot_id: &str, group_id: &str) -> bool {
        STORE.cooldown().is_cooling_down(&CooldownScope::Group { bot_id, group_id })
    }

    /// 检查群组内用户是否处于冷却期
    pub fn is_group_user_cooling_down(bot_id: &str, group_id: &str, user_id: &str) -> bool {
        STORE.cooldown().is_cooling_down(&CooldownScope::GroupUser { bot_id, group_id, user_id })
    }

    /// 设置好友消息冷却
    pub fn set_friend_cooldown(bot_id: &str, user_id: &str, duration: u64) {
        STORE.cooldown().set_cooldown(&CooldownScope::Friend { bot_id, user_id }, duration);
    }

    /// 设置群组冷却
    pub fn set_group_cooldown(bot_id: &str, group_id: &str, duration: u64) {
        STORE.cooldown().set_cooldown(&CooldownScope::Group { bot_id, group_id }, duration);
    }

    /// 设置群组内用户冷却
    pub fn set_group_user_cooldown(bot_id: &str, group_id: &str, user_id: &str, duration: u64) {
        STORE.cooldown().set_cooldown(&CooldownScope::GroupUser { bot_id, group_id, user_id }, duration);
    }

    /// 清除好友消息冷却
    pub fn clear_friend_cooldown(bot_id: &str, user_id: &str) {
        STORE.cooldown().clear_cooldown(&CooldownScope::Friend { bot_id, user_id });
    }

    /// 清除群组冷却
    pub fn clear_group_cooldown(bot_id: &str, group_id: &str) {
        STORE.cooldown().clear_cooldown(&CooldownScope::Group { bot_id, group_id });
    }

    /// 清除群组内用户冷却
    pub fn clear_group_user_cooldown(bot_id: &str, group_id: &str, user_id: &str) {
        STORE.cooldown().clear_cooldown(&CooldownScope::GroupUser { bot_id, group_id, user_id });
    }

    /// 清理所有过期的冷却记录
    pub fn cleanup_expired() {
        STORE.cooldown().cleanup_expired();
    }
}
