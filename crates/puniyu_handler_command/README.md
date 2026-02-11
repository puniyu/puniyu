# puniyu_command

`puniyu_command` 是 Puniyu 机器人框架的命令处理核心模块，负责解析、匹配和执行用户命令。

## 功能特性

### 🎯 命令处理

- **智能命令匹配**: 支持命令名称和别名匹配
- **参数解析**: 基于 clap 的强大参数解析系统
- **权限控制**: 支持主人权限验证
- **多种参数类型**: 字符串、整数、浮点数、布尔值

### 🛡️ 安全与控制

- **冷却系统**: 防止命令滥用的多级冷却机制
  - 全局冷却
  - 机器人级冷却
  - 好友/群组/群成员级冷却
- **权限验证**: 黑白名单系统控制访问权限
- **响应模式**: 多种机器人响应模式
  - 全部消息响应
  - 仅@机器人时响应
  - 仅使用别名时响应
  - @机器人或别名时响应
  - 仅主人可用

### 📝 消息处理

- **消息日志**: 详细的消息接收和处理日志
- **别名处理**: 智能识别和处理机器人别名
- **@检测**: 检测消息中是否@了机器人

## 核心组件

### CommandHandler

主要的命令处理器，实现了 `Handler` trait：

```rust
use puniyu_command::CommandHandler;

let handler = CommandHandler::default();
```

### 参数解析 

基于 clap 的参数解析系统，支持：

- 位置参数和命名参数
- 类型验证和转换
- 默认值设置
- 中文错误提示

### 命令匹配

智能命令匹配系统：

- 前缀识别
- 别名处理
- 参数提取

### 冷却系统 

多层级冷却控制：

```rust
// 检查是否在冷却期
if cooldown::is_cooling_down(&event) {
    return;
}

// 设置冷却
cooldown::set_cooldown(&event);
```

## 目录结构

```
src/
├── lib.rs              # 模块导出
├── handler.rs          # 主命令处理器
├── handler/
│   ├── arg.rs         # 参数解析
│   └── matcher.rs     # 命令匹配
├── config.rs          # 配置管理
├── cooldown.rs        # 冷却系统
├── cooldown/
│   ├── bot.rs         # 机器人级冷却
│   ├── friend.rs      # 好友级冷却
│   └── group.rs       # 群组级冷却
├── message.rs         # 消息处理
├── tools.rs           # 工具函数
└── error.rs           # 错误定义
```


## 使用示例

### 基本使用

```rust
use puniyu_command::CommandHandler;
use puniyu_types::handler::Handler;

#[tokio::main]
async fn main() {
    let handler = CommandHandler::default();

    // 检查是否匹配命令
    if handler.matches(&event) {
        // 处理命令
        handler.handle(&bot, &event).await.unwrap();
    }
}
```

### 命令执行流程

1. **消息接收**: 接收到消息事件
2. **权限检查**: 验证发送者权限（黑白名单）
3. **冷却检查**: 检查是否在冷却期内
4. **模式验证**: 根据配置的响应模式验证
5. **命令匹配**: 匹配命令名称或别名
6. **参数解析**: 解析命令参数
7. **权限验证**: 验证命令执行权限
8. **命令执行**: 执行注册的命令处理函数

## 配置支持

支持通过配置文件自定义：

- 机器人别名
- 响应模式
- 命令前缀
- 黑白名单
- 冷却时间


## 许可证

本项目采用 [LGPL](../../LICENSE) 许可证。
