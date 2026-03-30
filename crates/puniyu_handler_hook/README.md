# puniyu_handler_hook

puniyu Hook 处理器，将事件分发到所有已注册的 Hook。

## 特性

- 🔌 **统一入口**: 提供 `Handler`（即 `HookHandler`）实现 `puniyu_handler::Handler` trait
- 📊 **优先级调度**: Hook 按 `priority()` 升序依次执行
- 🎯 **事件过滤**: 仅将事件分发到类型匹配的 Hook
- 🛡️ **容错处理**: 单个 Hook 出错不影响后续 Hook 执行

## 示例

```rust,ignore
use puniyu_handler::Handler;
use puniyu_handler_hook::Handler as HookHandler;

let handler = HookHandler;
assert_eq!(handler.name(), "hook");
```

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。
