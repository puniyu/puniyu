# puniyu_config

统一的 puniyu 配置管理库，覆盖应用、Bot、群聊与好友场景。

## 特性

- 📦 提供 `AppConfig`、`BotConfig`、`GroupConfig`、`FriendConfig`
- 🔄 提供 `init()` 初始化配置目录、合并默认配置并启动配置监听
- 🧩 提供 `app_config()`、`bot_config()`、`group_config()`、`friend_config()` 统一访问入口
- ♻️ 支持全局配置与单项配置合并
- 📝 使用 TOML 配置文件

## 示例

```rust
use puniyu_config::{app_config, bot_config, friend_config, group_config, init};

init();

let app = app_config();
assert_eq!(app.prefix().as_deref(), Some("!"));

let bot = bot_config().bot("bot_001");
let group = group_config().group("group_123");
let friend = friend_config().friend("user_123");

println!("{}", app.logger().level());
println!("{}", bot.cd());
println!("{}", group.user_cd());
println!("{:?}", friend.alias());
```

## 配置文件

- `app.toml`: 应用级配置，包含日志、服务、名单和命令前缀
- `bot.toml`: Bot 全局配置和按 Bot ID 的配置
- `group.toml`: 群聊全局配置和按群号的配置
- `friend.toml`: 好友全局配置和按用户 ID 的配置

```toml
# group.toml
[global]
cd = 0
user_cd = 0
mode = 0
alias = []

[group.group_123]
cd = 10
user_cd = 5
alias = ["bot"]
```

## 响应模式

`ReactiveMode` 提供以下模式：

- `All`: 响应所有消息
- `AtBot`: 仅响应 @ Bot 的消息
- `Alias`: 仅响应使用别名的消息
- `AtOrAlias`: 响应 @ Bot 或使用别名的消息
- `Master`: 仅响应主人的消息

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。
