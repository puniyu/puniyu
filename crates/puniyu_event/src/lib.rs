//! # puniyu_event
//!
//! 事件类型定义库，提供聊天机器人中的各类事件类型系统。
//!
//! ## 概述
//!
//! `puniyu_event` 提供了完整的事件类型定义，用于处理聊天机器人中的各种事件。
//! 该库将事件分为三大类：
//!
//! - **消息事件（Message）** - 处理好友和群聊消息
//! - **通知事件（Notion）** - 处理各类通知（戳一戳、撤回、文件上传等）
//! - **请求事件（Request）** - 处理好友申请、群申请等请求
//!
//! ## 特性
//!
//! - 🎯 **类型安全** - 使用 Rust 类型系统确保事件处理的正确性
//! - 🔧 **统一接口** - 通过 trait 提供统一的事件访问接口
//! - 📦 **序列化支持** - 内置 serde 支持
//! - 🎨 **生命周期优化** - 使用生命周期参数避免不必要的内存分配
//! - 🔄 **丰富的事件类型** - 支持消息、通知、请求等多种事件类型
//!
//! ## 快速开始
//!
//! ### 处理事件
//!
//! ```rust,ignore
//! use puniyu_event::{Event, EventBase};
//!
//! fn handle_event(event: Event) {
//!     match event {
//!         Event::Message(msg) => {
//!             // 处理消息事件
//!             let texts = msg.get_text();
//!             println!("收到消息: {:?}", texts);
//!         }
//!         Event::Notion(notion) => {
//!             // 处理通知事件
//!             println!("收到通知");
//!         }
//!         Event::Request(request) => {
//!             // 处理请求事件
//!             println!("收到请求");
//!         }
//!     }
//! }
//! ```
//!
//! ### 处理好友消息
//!
//! ```rust,ignore
//! use puniyu_event::message::{FriendMessage, MessageBase};
//!
//! fn handle_friend_message(msg: &FriendMessage) {
//!     // 获取消息文本
//!     let texts = msg.get_text();
//!     println!("收到好友消息: {:?}", texts);
//!     
//!     // 获取发送者信息
//!     let sender = msg.sender();
//!     println!("发送者: {}", sender.user_id());
//!     
//!     // 判断是否为主人
//!     if msg.is_master() {
//!         println!("这是主人发送的消息");
//!     }
//! }
//! ```
//!
//! ### 处理群消息
//!
//! ```rust,ignore
//! use puniyu_event::message::{GroupMessage, MessageBase};
//!
//! fn handle_group_message(msg: &GroupMessage) {
//!     // 获取群 ID
//!     let group_id = msg.group_id();
//!     println!("群 ID: {}", group_id);
//!     
//!     // 判断发送者是否为管理员
//!     if msg.is_admin() {
//!         println!("管理员发送的消息");
//!     }
//!     
//!     // 获取消息文本
//!     let texts = msg.get_text();
//!     println!("群消息: {:?}", texts);
//! }
//! ```
//!
//! ### 使用 EventBase trait
//!
//! ```rust,ignore
//! use puniyu_event::EventBase;
//!
//! fn print_event_info<E>(event: &E)
//! where
//!     E: EventBase
//! {
//!     println!("事件 ID: {}", event.event_id());
//!     println!("时间戳: {}", event.time());
//!     println!("用户 ID: {}", event.user_id());
//!     println!("机器人 ID: {}", event.self_id());
//! }
//! ```

pub mod message;
pub mod notion;
pub mod request;
mod types;
#[doc(inline)]
pub use types::*;
