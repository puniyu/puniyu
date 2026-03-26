# puniyu_adapter_core

适配器核心库，提供适配器 API 接口和类型定义。

## 概述

`puniyu_adapter_core` 是 Puniyu 框架的适配器核心库，定义了适配器的标准接口和类型系统。该库提供了：

- **适配器 API 接口** - 消息、群组、好友、账户等核心功能的异步 API
- **类型定义** - 适配器信息、消息类型、用户信息、群组信息等
- **统一抽象** - 为不同平台的适配器提供统一的接口规范

## 特性

- 🎯 **异步 API** - 基于 async-trait 的异步接口设计
- 🔧 **类型安全** - 使用 Rust 类型系统确保 API 调用的正确性
- 🔄 **统一接口** - 为不同平台提供一致的 API 抽象
- 📦 **序列化支持** - 内置 serde 支持
- 🎨 **灵活扩展** - 支持自定义适配器实现

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
puniyu_adapter_core = "*"
```

## 快速开始

### 实现适配器

```rust
use puniyu_adapter_core::{Adapter, AdapterApi, AdapterInfo};
use async_trait::async_trait;

struct MyAdapter {
    info: AdapterInfo,
    api: AdapterApi,
}

#[async_trait]
impl Adapter for MyAdapter {
    type Context = ();

    fn info(&self) -> &AdapterInfo {
        &self.info
    }

    fn api(&self) -> &AdapterApi {
        &self.api
    }
}
```

### 使用适配器 API

```rust
use puniyu_adapter_core::api::{MessageApi, GroupApi, FriendApi, AccountApi};

// 消息 API
async fn send_message(api: &impl MessageApi) {
    // 发送消息
    // api.send_msg(...).await?;
}

// 群组 API
async fn get_group_list(api: &impl GroupApi) {
    let groups = api.get_group_list().await?;
    for group in groups {
        println!("群组: {}", group.group_name);
    }
}

// 好友 API
async fn get_friend_list(api: &impl FriendApi) {
    let friends = api.get_friend_list().await?;
    for friend in friends {
        println!("好友: {}", friend.user_name);
    }
}
```

### 创建适配器信息

```rust
use puniyu_adapter_core::{adapter_info, AdapterPlatform, AdapterProtocol};
use puniyu_version::Version;

let info = adapter_info!(
    name: "my_adapter",
    platform: AdapterPlatform::QQ,
    protocol: AdapterProtocol::Console,
    VERSION: Version::new(1, 0, 0),
);
```

## API 接口

### MessageApi - 消息 API

提供消息发送、撤回、获取等功能：

- `send_msg` - 发送消息
- `recall_msg` - 撤回消息
- `get_msg` - 获取消息
- `get_history_msg` - 获取历史消息

### GroupApi - 群组 API

提供群组管理功能：

- `get_group_avatar` - 获取群头像
- `get_group_info` - 获取群信息
- `get_group_list` - 获取群列表
- `get_group_member_list` - 获取群成员列表

### FriendApi - 好友 API

提供好友管理功能：

- `get_user_avatar` - 获取用户头像
- `get_friend_list` - 获取好友列表
- `set_friend_apply` - 处理好友申请

### AccountApi - 账户 API

提供账户管理功能：

- `set_avatar` - 设置头像

## 类型定义

### AdapterInfo - 适配器信息

包含适配器的基本信息：

- `name` - 适配器名称
- `platform` - 平台类型（QQ、WeChat 等）
- `protocol` - 协议类型（Console、HTTP、WebSocket 等）
- `standard` - 标准类型（OneBot、Satori 等）
- `communication` - 通信方式（正向、反向）
- `VERSION` - 版本号

### MessageType - 消息类型

支持多种消息标识方式：

- `MessageId(String)` - 消息 ID
- `MessageSeq(u64)` - 消息序列号

### UserInfo - 用户信息

包含用户的基本信息：

- `user_id` - 用户 ID
- `user_name` - 用户名
- `user_remark` - 用户备注

### GroupInfo - 群组信息

包含群组的基本信息：

- `group_id` - 群 ID
- `group_name` - 群名称
- `member_count` - 成员数量
- `max_member_count` - 最大成员数量

## 平台支持

支持的平台类型：

- QQ
- WeChat
- Telegram
- Discord
- Kook
- DoDo
- Slack
- Line
- Viber
- WhatsApp

## 协议支持

支持的协议类型：

- Console - 控制台
- HTTP - HTTP 协议
- WebSocket - WebSocket 协议
- WebHook - WebHook 协议

## 标准支持

支持的标准类型：

- OneBot - OneBot 标准
- Satori - Satori 标准
- Kritor - Kritor 标准

## 文档

完整的 API 文档请访问 [docs.rs](https://docs.rs/puniyu_adapter_core)。

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。

## 相关链接

- [GitHub 仓库](https://github.com/puniyu/puniyu)
- [Puniyu 框架](https://github.com/puniyu/puniyu)
- [puniyu_adapter](https://docs.rs/puniyu_adapter) - 适配器注册库
