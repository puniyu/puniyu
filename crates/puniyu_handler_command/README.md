# puniyu_handler_command

统一的 puniyu 命令处理器，覆盖命令匹配、权限检查、冷却控制与执行分发场景。

## 特性

- 🧩 提供 `CommandHandler`
- 🔍 结合 `puniyu_command_parser` 完成命令文本解析
- 🔐 支持权限检查、响应模式判断和主人命令校验
- ⏱️ 支持全局、Bot、好友、群组与群成员冷却

## 示例

```rust,ignore
use puniyu_handler::Handler;
use puniyu_handler_command::CommandHandler;

let handler = CommandHandler::default();
assert_eq!(handler.name(), "command");
```

## 流程

- 读取消息文本并记录日志
- 检查黑白名单和响应模式
- 检查并设置命令冷却
- 调用 `puniyu_command_parser` 解析命令
- 按优先级执行已注册命令

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。
