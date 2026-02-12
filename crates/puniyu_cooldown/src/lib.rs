//! # puniyu_cooldown
//!
//! Puniyu 冷却管理库，提供命令和功能的冷却时间控制。
//!
//! ## 概述
//!
//! `puniyu_cooldown` 提供了灵活的冷却时间管理系统，支持多种冷却范围：
//!
//! - **全局冷却** - 所有用户共享的冷却时间
//! - **机器人级别** - 特定机器人的冷却时间
//! - **好友级别** - 特定好友的冷却时间
//! - **群组级别** - 特定群组的冷却时间
//! - **群成员级别** - 特定群组中特定成员的冷却时间
//!
//! ## 功能特性
//!
//! - 🎯 **多级冷却** - 支持全局、机器人、好友、群组和群成员级别
//! - ⏱️ **灵活时间** - 使用 `Duration` 设置任意冷却时长
//! - 🔄 **自动清理** - 支持清理过期的冷却记录
//! - 🔒 **线程安全** - 使用 `RwLock` 保证并发安全
//! - 📊 **状态查询** - 快速检查是否处于冷却期
//!
//! ## 使用示例
//!
//! ### 基本用法
//!
//! ```rust
//! use puniyu_cooldown::{CooldownRegistry, CooldownScope};
//! use std::time::Duration;
//!
//! // 检查全局冷却
//! let scope = CooldownScope::Global;
//! if CooldownRegistry::is_cooling_down(&scope) {
//!     println!("正在冷却中");
//! } else {
//!     // 设置 60 秒冷却
//!     CooldownRegistry::set_cooldown(&scope, Duration::from_secs(60)).unwrap();
//!     println!("已设置冷却");
//! }
//! ```
//!
//! ### 好友级别冷却
//!
//! ```rust
//! use puniyu_cooldown::{CooldownRegistry, CooldownScope};
//! use std::time::Duration;
//!
//! let scope = CooldownScope::Friend {
//!     bot_id: "123456",
//!     user_id: "789012",
//! };
//!
//! if !CooldownRegistry::is_cooling_down(&scope) {
//!     // 执行命令
//!     println!("执行命令");
//!     
//!     // 设置 30 秒冷却
//!     CooldownRegistry::set_cooldown(&scope, Duration::from_secs(30)).unwrap();
//! }
//! ```
//!
//! ### 群组级别冷却
//!
//! ```rust
//! use puniyu_cooldown::{CooldownRegistry, CooldownScope};
//! use std::time::Duration;
//!
//! let scope = CooldownScope::Group {
//!     bot_id: "123456",
//!     group_id: "456789",
//! };
//!
//! CooldownRegistry::set_cooldown(&scope, Duration::from_secs(120)).unwrap();
//! ```
//!
//! ### 清理过期记录
//!
//! ```rust
//! use puniyu_cooldown::CooldownRegistry;
//!
//! // 定期清理过期的冷却记录
//! CooldownRegistry::cleanup_expired();
//! ```

mod types;
#[doc(inline)]
pub use types::CooldownScope;
mod registry;
pub use registry::CooldownRegistry;
