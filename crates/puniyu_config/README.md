# puniyu_config

puniyu 框架的配置管理模块，提供统一的配置读取、热重载和管理功能。

## 功能特性

- ✨ **多层级配置**: 支持应用级、Bot级、群组级和好友级配置
- 🔥 **热重载**: 自动监听配置文件变化并实时更新
- 🛡️ **类型安全**: 使用强类型配置结构，避免运行时错误
- 📝 **默认值**: 所有配置项都有合理的默认值
- 🔀 **配置合并**: 支持默认配置与用户配置的智能合并
- 📂 **TOML格式**: 使用易读易写的 TOML 配置文件格式

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
puniyu_config = "*"
```

## 配置层级

### 1. 应用配置 (AppConfig)

全局应用设置，包括日志、服务器、适配器等。

**配置文件**: `app.toml`

```toml
# 命令前缀
prefix = "!"

# Bot 主人列表
masters = ["console", "admin123"]

[logger]
enable_file = true
level = "info"
retention_days = 7

[server]
host = "127.0.0.1"
port = 33720

[adapter]
console = true
server = true
```

### 2. Bot 配置 (BotConfig)

每个 Bot 实例的独立配置。

**配置文件**: `bot.toml`

```toml
[global]
cd = 0
mode = 0
alias = []

[bot.bot_001]
cd = 5
mode = 1
alias = ["小助手", "bot"]

[bot.bot_002]
cd = 0
mode = 0
alias = ["助手"]
```

### 3. 群组配置 (GroupConfig)

群聊相关的配置，支持全局和单独群组配置。

**配置文件**: `group.toml`

```toml
[global]
cd = 0
user_cd = 0
mode = 0
alias = []

[group.group_123]
cd = 10
user_cd = 5
mode = 1
alias = ["bot"]

[group.group_456]
cd = 0
user_cd = 0
mode = 0
alias = []
```

### 4. 好友配置 (FriendConfig)

好友相关的配置，支持全局和单独好友配置。

**配置文件**: `friend.toml`

```toml
[global]
cd = 0
mode = 0
alias = []

[friend.user_789]
cd = 5
mode = 0
alias = ["bot"]
```

## 使用示例

### 基础使用

```rust
use puniyu_config::{Config, init_config, start_config_watcher};

fn main() {
    // 初始化配置系统
    init_config();

    // 启动配置文件监听器（可选，用于热重载）
    start_config_watcher();

    // 获取应用配置
    let app_config = Config::app();
    println!("日志级别: {}", app_config.logger().level());
    println!("服务器地址: {}:{}",
        app_config.server().host(),
        app_config.server().port()
    );
    println!("命令前缀: {}", app_config.prefix());

    // 获取 Bot 配置
    let bot_config = Config::bot();
    let bot_option = bot_config.bot("bot_001");
    println!("Bot CD: {}", bot_option.cd());
    println!("Bot 别名: {:?}", bot_option.alias());

    // 获取群组配置
    let group_config = Config::group();
    let group_option = group_config.group("group_123");
    println!("群组 CD: {}", group_option.cd());
    println!("用户 CD: {}", group_option.user_cd());

    // 获取好友配置
    let friend_config = Config::friend();
    let friend_option = friend_config.friend("user_789");
    println!("好友 CD: {}", friend_option.cd());
}
```

### 响应模式

使用 `ReactiveMode` 枚举定义Bot的响应行为：

```rust
use puniyu_config::ReactiveMode;

let mode = ReactiveMode::AtBot;  // 仅响应@Bot的消息

match mode {
    ReactiveMode::All => println!("响应所有消息"),
    ReactiveMode::AtBot => println!("仅响应@Bot的消息"),
    ReactiveMode::Alias => println!("仅响应使用别名的消息"),
    ReactiveMode::AtOrAlias => println!("响应@Bot或使用别名的消息"),
    ReactiveMode::Master => println!("仅响应主人的消息"),
}
```

### 配置热重载

配置模块会自动监听配置文件的变化：

```rust
use puniyu_config::{init_config, start_config_watcher, Config};

// 初始化并启动监听
init_config();
start_config_watcher();

// 当配置文件被修改时，会自动重新加载
// 无需重启应用即可应用新配置
loop {
    let config = Config::app();
    println!("当前前缀: {}", config.prefix());

    std::thread::sleep(std::time::Duration::from_secs(5));
}
```

### 获取配置列表

```rust
use puniyu_config::Config;

// 获取所有 Bot 配置
let bot_config = Config::bot();
for (bot_id, bot_option) in bot_config.list() {
    println!("Bot ID: {}, CD: {}, 别名: {:?}",
        bot_id, bot_option.cd(), bot_option.alias());
}

// 获取所有群组配置
let group_config = Config::group();
for group_option in group_config.list() {
    println!("群组 CD: {}, 用户 CD: {}",
        group_option.cd(), group_option.user_cd());
}

// 获取所有好友配置
let friend_config = Config::friend();
for friend_option in friend_config.list() {
    println!("好友 CD: {}", friend_option.cd());
}
```

## 配置文件位置

配置文件默认存储在配置目录中，具体位置取决于 `puniyu_path` 模块的 `CONFIG_DIR` 设置。

## 响应模式说明

| 模式        | 值  | 说明                     |
| ----------- | --- | ------------------------ |
| `All`       | 0   | 响应所有消息             |
| `AtBot`     | 1   | 仅响应@Bot的消息         |
| `Alias`     | 2   | 仅响应使用别名的消息     |
| `AtOrAlias` | 3   | 响应@Bot或使用别名的消息 |
| `Master`    | 4   | 仅响应主人的消息         |

## 配置继承机制

puniyu_config 支持配置继承，特定配置会自动继承全局配置。这意味着你只需要在特定配置中设置需要覆盖的字段，其他字段会自动使用全局配置的值。

### 继承规则

1. **全局配置作为基础**: `global` 配置定义所有实例的默认值
2. **特定配置覆盖**: 特定 ID 的配置只需设置需要修改的字段
3. **自动合并**: 获取配置时会自动合并全局配置和特定配置

### 继承示例

```toml
# bot.toml
[global]
cd = 0
mode = 0
alias = ["bot"]

[bot.bot_001]
# 只覆盖 cd，mode 和 alias 继承全局配置
cd = 5

[bot.bot_002]
# 只覆盖 mode 和 alias，cd 继承全局配置
mode = 1
alias = ["小助手", "助手"]
```

在上面的例子中：

- `bot_001` 的配置为: `cd=5, mode=0, alias=["bot"]`
- `bot_002` 的配置为: `cd=0, mode=1, alias=["小助手", "助手"]`

### 群组配置继承

```toml
# group.toml
[global]
cd = 0
user_cd = 0
mode = 0
alias = []

[group.group_123]
# 只设置群组 CD，其他继承全局配置
cd = 10

[group.group_456]
# 设置群组 CD 和用户 CD
cd = 10
user_cd = 5
```

### 好友配置继承

```toml
# friend.toml
[global]
cd = 0
mode = 0
alias = ["bot"]

[friend.user_789]
# 只覆盖 cd，其他继承全局配置
cd = 5
```

### 代码中使用继承

```rust
use puniyu_config::Config;

let bot_config = Config::bot();

// 获取 bot_001 的配置，自动合并全局配置
let bot_001 = bot_config.bot("bot_001");
println!("CD: {}", bot_001.cd());        // 输出: 5
println!("Mode: {:?}", bot_001.mode());  // 输出: All (继承自全局)
println!("Alias: {:?}", bot_001.alias()); // 输出: ["bot"] (继承自全局)

// 获取 bot_002 的配置
let bot_002 = bot_config.bot("bot_002");
println!("CD: {}", bot_002.cd());        // 输出: 0 (继承自全局)
println!("Mode: {:?}", bot_002.mode());  // 输出: AtBot
println!("Alias: {:?}", bot_002.alias()); // 输出: ["小助手", "助手"]
```

## 配置优先级

配置继承遵循以下优先级：

1. **特定配置**: 针对特定 ID 的配置（如 `bot.bot_001`、`group.group_123`）中显式设置的字段
2. **全局配置**: 全局默认配置（`global`）中的字段
3. **代码默认值**: 硬编码的默认值（当全局配置也未设置时）

示例：

```rust
let bot_config = Config::bot();

// 配置继承示例：
// global: cd=0, mode=0, alias=["bot"]
// bot.bot_001: cd=5 (只设置了 cd)
//
// 最终 bot_001 的配置为:
// cd=5 (来自特定配置), mode=0 (继承全局), alias=["bot"] (继承全局)
let bot_option = bot_config.bot("bot_001");
```

## 配置项说明

### Bot 配置项

| 字段    | 类型           | 默认值  | 说明                             |
| ------- | -------------- | ------- | -------------------------------- |
| `cd`    | `u64`          | 0       | 消息冷却时间（秒），0 表示无冷却 |
| `mode`  | `ReactiveMode` | 0 (All) | 响应模式                         |
| `alias` | `Vec<String>`  | []      | Bot 别名列表                     |

### 群组配置项

| 字段      | 类型           | 默认值  | 说明                 |
| --------- | -------------- | ------- | -------------------- |
| `cd`      | `u64`          | 0       | 群组级冷却时间（秒） |
| `user_cd` | `u64`          | 0       | 用户级冷却时间（秒） |
| `mode`    | `ReactiveMode` | 0 (All) | 响应模式             |
| `alias`   | `Vec<String>`  | []      | Bot 别名列表         |

### 好友配置项

| 字段    | 类型           | 默认值  | 说明               |
| ------- | -------------- | ------- | ------------------ |
| `cd`    | `u64`          | 0       | 消息冷却时间（秒） |
| `mode`  | `ReactiveMode` | 0 (All) | 响应模式           |
| `alias` | `Vec<String>`  | []      | Bot 别名列表       |

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。

## 相关链接

- [GitHub 仓库](https://github.com/puniyu/puniyu)
- [Puniyu 框架](https://github.com/puniyu/puniyu)
