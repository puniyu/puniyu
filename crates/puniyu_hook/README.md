# puniyu_hook

钩子系统库，提供统一的 `Hook` 接口和钩子类型定义。

## 特性

- 🎣 **统一接口**: 通过 `Hook` trait 定义名称、类型、优先级和执行逻辑
- ⚡ **异步执行**: 基于 `async_trait` 的异步钩子调用
- 🧩 **类型系统**: 提供 `HookType`、`HookEventType`、`StatusType` 等类型
- 📚 **注册管理**: 启用 `registry` feature 后可使用 `HookRegistry`

## 示例

```rust,ignore
use async_trait::async_trait;
use puniyu_context::EventContext;
use puniyu_hook::{Hook, HookEventType, HookType};

struct LogHook;

#[async_trait]
impl Hook for LogHook {
    fn name(&self) -> &'static str { "log_hook" }
    fn r#type(&self) -> &HookType { &HookType::Event(HookEventType::Message) }
    fn priority(&self) -> u32 { 100 }
    async fn run(&self, _ctx: Option<&EventContext>) -> puniyu_error::Result { Ok(()) }
}
```

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。
