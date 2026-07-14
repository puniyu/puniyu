//! # puniyu_cooldown
//!
//! 面向消息处理流程的全局冷却器，覆盖全局、机器人、好友、群组与群成员场景。
//!
//! ## 特性
//!
//! - 支持设置、状态检查、剩余时间查询与移除
//! - 支持原子判定并开始固定冷却窗口
//! - 使用单调时钟，不受系统时间调整影响
//! - 查询或重置作用域时移除过期记录
//!
//! ## 示例
//!
//! ```text
//! use puniyu_cooldown::{Cooldown, CooldownScope, CooldownState};
//! use std::time::Duration;
//!
//! let scope = CooldownScope::friend("123456", "789012");
//! match Cooldown::check_and_set(&scope, Duration::from_secs(30)) {
//!     CooldownState::Ready => println!("继续处理消息"),
//!     CooldownState::CoolingDown { remaining } => {
//!         println!("仍需等待 {remaining:?}");
//!     }
//! }
//! ```

mod types;
#[doc(inline)]
pub use types::*;
mod cooldown;
mod registry;
#[doc(inline)]
pub use cooldown::Cooldown;
