# puniyu_bot

机器人实例库，提供机器人的统一类型定义和注册表管理。

## 概述

`puniyu_bot` 提供了机器人实例的类型定义，封装了适配器信息、API 接口和账户信息。该库还提供了可选的注册表功能，用于管理多个机器人实例。

## 特性

- 🎯 **统一封装** - 将适配器、API 和账户信息封装在一个类型中
- 🔧 **简单易用** - 提供简洁的 API 访问机器人信息
- 📦 **类型安全** - 使用 Rust 类型系统确保数据正确性
- 🔄 **可克隆** - 支持克隆操作，方便在多处使用
- 📋 **注册表管理** - 可选的全局注册表功能，管理多个机器人实例

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
puniyu_bot = "*"
```

启用注册表功能：

```toml
[dependencies]
puniyu_bot = { VERSION = "*", features = ["registry"] }
```

## 快速开始

### 创建机器人实例

```rust
use puniyu_bot::Bot;
use puniyu_account::AccountInfo;
use puniyu_adapter_core::api::AdapterApi;
use puniyu_adapter_core::adapter_info;
use puniyu_adapter_core::types::info::{AdapterPlatform, AdapterProtocol};
use puniyu_version::Version;

// 创建适配器信息
let adapter = adapter_info!(
    name: "my_adapter",
    platform: AdapterPlatform::QQ,
    protocol: AdapterProtocol::Console,
    VERSION: Version::new(1, 0, 0),
);

// 创建 API 实例
let api = AdapterApi::default();

// 创建账户信息
let account = AccountInfo {
    uin: "123456".to_string(),
    name: "MyBot".to_string(),
    avatar: "".to_string(),
};

// 创建机器人实例
let bot = Bot::new(adapter, api, account);
```

### 访问机器人信息

```rust
// 获取适配器信息
let adapter = bot.adapter();
println!("适配器名称: {}", adapter.name);
println!("平台: {}", adapter.platform);

// 获取账户信息
let account = bot.account();
println!("机器人 UIN: {}", account.uin);
println!("机器人昵称: {}", account.name);
```

### 使用 API 接口

```rust
// 获取 API
let api = bot.api();

// 发送消息
api.message().send_msg(&contact, &message).await?;

// 获取群列表
let groups = api.group().get_group_list().await?;

// 获取好友列表
let friends = api.friend().get_friend_list().await?;
```

## 使用注册表

启用 `registry` 特性后，可以使用全局注册表管理多个机器人实例。

### 注册机器人

```rust
use puniyu_bot::{Bot, BotRegistry};

// 注册机器人
let index = BotRegistry::register(bot)?;
println!("机器人已注册，索引: {}", index);
```

### 查询机器人

```rust
// 通过索引获取
if let Some(bot) = BotRegistry::get_with_index(index) {
    println!("找到机器人: {}", bot.account().uin);
}

// 通过 UIN 获取
let bots = BotRegistry::get_with_bot_id("123456");
for bot in bots {
    println!("机器人: {}", bot.account().name);
}

// 使用统一接口
let bots = BotRegistry::get(index);  // 使用索引
let bots = BotRegistry::get("123456");  // 使用 UIN

// 获取所有机器人
let all_bots = BotRegistry::all();
println!("共有 {} 个机器人", all_bots.len());
```

### 注销机器人

```rust
// 使用索引注销
BotRegistry::unregister(index)?;

// 使用 UIN 注销
BotRegistry::unregister("123456")?;

// 或使用具体方法
BotRegistry::unregister_with_index(index)?;
BotRegistry::unregister_with_bot_id("123456")?;
```

## Bot 结构

`Bot` 结构包含以下字段：

| 字段      | 类型          | 说明           |
| --------- | ------------- | -------------- |
| `adapter` | `AdapterInfo` | 适配器信息     |
| `api`     | `AdapterApi`  | 适配器 API     |
| `account` | `AccountInfo` | 机器人账户信息 |

## Bot 方法

### new

创建新的机器人实例。

```rust
pub fn new(adapter: AdapterInfo, api: AdapterApi, account: AccountInfo) -> Self
```

### adapter

获取适配器信息的引用。

```rust
pub fn adapter(&self) -> &AdapterInfo
```

### api

获取适配器 API 的引用。

```rust
pub fn api(&self) -> &AdapterApi
```

### account

获取账户信息的引用。

```rust
pub fn account(&self) -> &AccountInfo
```

## BotRegistry 方法

### register

注册机器人到注册表。

```rust
pub fn register(bot: Bot) -> Result<u64, Error>
```

### unregister

从注册表注销机器人（支持索引或 UIN）。

```rust
pub fn unregister<B: Into<BotId>>(bot_id: B) -> Result<(), Error>
```

### get

获取机器人实例（支持索引或 UIN）。

```rust
pub fn get<T: Into<BotId>>(bot_id: T) -> Vec<Bot>
```

### get_with_index

使用索引获取机器人。

```rust
pub fn get_with_index(index: u64) -> Option<Bot>
```

### get_with_bot_id

使用 UIN 获取机器人。

```rust
pub fn get_with_bot_id(self_id: &str) -> Vec<Bot>
```

### all

获取所有已注册的机器人。

```rust
pub fn all() -> Vec<Bot>
```

## BotId 类型

`BotId` 枚举用于标识机器人：

```rust
pub enum BotId<'b> {
    Index(u64),      // 注册表索引
    SelfId(&'b str), // 机器人 UIN
}
```

支持从 `u64` 和 `&str` 自动转换：

```rust
let id1: BotId = 123u64.into();
let id2: BotId = "123456".into();
```

## 完整示例

```rust
use puniyu_bot::{Bot, BotRegistry};
use puniyu_account::AccountInfo;
use puniyu_adapter_core::api::AdapterApi;
use puniyu_adapter_core::adapter_info;
use puniyu_adapter_core::types::info::{AdapterPlatform, AdapterProtocol};
use puniyu_version::Version;

// 创建机器人
let adapter = adapter_info!(
    name: "napcat_adapter",
    platform: AdapterPlatform::QQ,
    protocol: AdapterProtocol::NapCat,
    VERSION: Version::new(1, 0, 0),
);

let api = AdapterApi::default();

let account = AccountInfo {
    uin: "123456789".to_string(),
    name: "MyBot".to_string(),
    avatar: "https://example.com/avatar.jpg".to_string(),
};

let bot = Bot::new(adapter, api, account);

// 使用机器人
println!("机器人 UIN: {}", bot.account().uin);
println!("适配器: {}", bot.adapter().name);

// 注册到注册表（需要 registry 特性）
#[cfg(feature = "registry")]
{
    let index = BotRegistry::register(bot.clone())?;

    // 查询机器人
    if let Some(bot) = BotRegistry::get_with_index(index) {
        println!("找到机器人: {}", bot.account().name);
    }

    // 注销机器人
    BotRegistry::unregister(index)?;
}
```

## 特性标志

### registry

启用机器人注册表功能。

```toml
[dependencies]
puniyu_bot = { VERSION = "*", features = ["registry"] }
```

启用后可以使用 `BotRegistry` 来管理多个机器人实例。

## 文档

完整的 API 文档请访问 [docs.rs](https://docs.rs/puniyu_bot)。

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。

## 相关链接

- [GitHub 仓库](https://github.com/puniyu/puniyu)
- [Puniyu 框架](https://github.com/puniyu/puniyu)
- [puniyu_account](https://docs.rs/puniyu_account) - 账户信息库
- [puniyu_adapter_core](https://docs.rs/puniyu_adapter_core) - 适配器核心库
