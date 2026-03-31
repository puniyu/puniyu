//! # puniyu_cooldown
//!
//! 统一的冷却管理类型，覆盖全局、机器人、好友、群组与群成员场景。
//!
//! ## 特性
//!
//! - 提供 `CooldownScope`
//! - 提供 `CooldownRegistry`
//! - 支持任意 `Duration` 冷却时长
//! - 支持清理过期冷却记录
//!
//! ## 示例
//!
//! ```rust
//! use puniyu_cooldown::{CooldownRegistry, CooldownScope};
//! use std::time::Duration;
//!
//! let scope = CooldownScope::Global;
//! if !CooldownRegistry::is_cooling_down(&scope) {
//!     CooldownRegistry::set_cooldown(&scope, Duration::from_secs(60)).unwrap();
//! }
//! ```
//!
//! ```rust
//! use puniyu_cooldown::{CooldownRegistry, CooldownScope};
//! use std::time::Duration;
//!
//! let scope = CooldownScope::Friend {
//!     bot_id: "123456",
//!     user_id: "789012",
//! };
//! CooldownRegistry::set_cooldown(&scope, Duration::from_secs(30)).unwrap();
//! CooldownRegistry::clear_cooldown(&scope).unwrap();
//! CooldownRegistry::cleanup_expired();
//! ```

mod types;
#[doc(inline)]
pub use types::*;
mod registry;
#[doc(inline)]
pub use registry::*;
