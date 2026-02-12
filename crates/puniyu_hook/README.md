# puniyu_hook

钩子系统库，提供事件钩子的定义和管理。

## 概述

`puniyu_hook` 提供了钩子系统的核心功能，允许在特定事件发生时执行自定义逻辑。钩子可以用于日志记录、权限检查、数据处理等场景。

## 特性

- 🎯 **事件驱动** - 基于事件触发的钩子系统
- 🔧 **优先级控制** - 支持钩子执行优先级设置
- 🔄 **异步支持** - 基于 async/await 的异步钩子
- 📦 **类型安全** - 完整的类型系统确保编译时安全
- 🎨 **灵活扩展** - 易于实现自定义钩子

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
puniyu_hook = "*"
```

## 快速开始

### 实现钩子

```rust
use puniyu_hook::{Hook, types::{HookType, HookEventType}};
use puniyu_context::EventContext;
use async_trait::async_trait;

struct LogHook;

#[async_trait]
impl Hook for LogHook {
    fn name(&self) -> &'static str {
        "log_hook"
    }

    fn r#type(&self) -> &HookType {
        &HookType::Event(HookEventType::Message)
    }

    fn rank(&self) -> u32 {
        100
    }

    async fn run(&self, ctx: Option<&EventContext>) -> puniyu_error::Result {
        if let Some(ctx) = ctx {
            println!("收到消息事件");
        }
        Ok(())
    }
}
```

### 使用钩子

```rust
async fn execute_hook(hook: &dyn Hook, ctx: &EventContext) {
    match hook.run(Some(ctx)).await {
        Ok(_) => println!("钩子执行成功"),
        Err(e) => eprintln!("钩子执行失败: {}", e),
    }
}
```

## Hook Trait

`Hook` trait 定义了钩子的基本接口：

### 方法

| 方法     | 说明           | 返回值               | 必需 |
| -------- | -------------- | -------------------- | ---- |
| `name`   | 获取钩子名称   | `&'static str`       | 是   |
| `r#type` | 获取钩子类型   | `&HookType`          | 是   |
| `rank`   | 获取钩子优先级 | `u32`                | 是   |
| `run`    | 执行钩子逻辑   | `Result<()>` (async) | 是   |

## 钩子类型

### HookType

钩子类型枚举，定义钩子的触发时机：

```rust
pub enum HookType {
    Event(HookEventType),  // 事件钩子
    Status(StatusType),    // 状态钩子
}
```

### HookEventType

事件钩子类型：

- `Message` - 消息事件
- `Notion` - 通知事件
- `Request` - 请求事件
- `All` - 所有事件

## 优先级

钩子的优先级由 `rank()` 方法返回的数值决定：

- 数值越小，优先级越高
- 默认优先级为 100
- 优先级相同时，按注册顺序执行

```rust
impl Hook for HighPriorityHook {
    fn rank(&self) -> u32 {
        10  // 高优先级
    }
}

impl Hook for LowPriorityHook {
    fn rank(&self) -> u32 {
        200  // 低优先级
    }
}
```

## 完整示例

### 权限检查钩子

```rust
use puniyu_hook::{Hook, types::{HookType, HookEventType}};
use puniyu_context::EventContext;
use async_trait::async_trait;

struct PermissionHook;

#[async_trait]
impl Hook for PermissionHook {
    fn name(&self) -> &'static str {
        "permission_hook"
    }

    fn r#type(&self) -> &HookType {
        &HookType::Event(HookEventType::Message)
    }

    fn rank(&self) -> u32 {
        10  // 高优先级，先执行权限检查
    }

    async fn run(&self, ctx: Option<&EventContext>) -> puniyu_error::Result {
        if let Some(ctx) = ctx {
            if let Some(msg_ctx) = ctx.as_message() {
                // 检查权限
                if !msg_ctx.is_master() {
                    return Err(puniyu_error::Error::msg("权限不足"));
                }
            }
        }
        Ok(())
    }
}
```

### 日志记录钩子

```rust
use puniyu_hook::{Hook, types::{HookType, HookEventType}};
use puniyu_context::EventContext;
use async_trait::async_trait;

struct LoggingHook;

#[async_trait]
impl Hook for LoggingHook {
    fn name(&self) -> &'static str {
        "logging_hook"
    }

    fn r#type(&self) -> &HookType {
        &HookType::Event(HookEventType::All)
    }

    fn rank(&self) -> u32 {
        200  // 低优先级，最后记录日志
    }

    async fn run(&self, ctx: Option<&EventContext>) -> puniyu_error::Result {
        if let Some(ctx) = ctx {
            println!("事件类型: {:?}", ctx.event());

            if ctx.is_message() {
                println!("消息事件");
            } else if ctx.is_notice() {
                println!("通知事件");
            } else if ctx.is_request() {
                println!("请求事件");
            }
        }
        Ok(())
    }
}
```

### 数据处理钩子

```rust
use puniyu_hook::{Hook, types::{HookType, HookEventType}};
use puniyu_context::EventContext;
use async_trait::async_trait;

struct DataProcessHook {
    processed_count: std::sync::Arc<std::sync::atomic::AtomicU64>,
}

#[async_trait]
impl Hook for DataProcessHook {
    fn name(&self) -> &'static str {
        "data_process_hook"
    }

    fn r#type(&self) -> &HookType {
        &HookType::Event(HookEventType::Message)
    }

    fn rank(&self) -> u32 {
        100
    }

    async fn run(&self, ctx: Option<&EventContext>) -> puniyu_error::Result {
        if let Some(ctx) = ctx {
            if let Some(msg_ctx) = ctx.as_message() {
                // 处理消息数据
                let texts = msg_ctx.get_text();
                println!("消息内容: {:?}", texts);

                // 更新计数
                self.processed_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        }
        Ok(())
    }
}
```

## 钩子执行流程

1. 根据事件类型筛选匹配的钩子
2. 按优先级（rank）排序钩子
3. 依次执行钩子的 `run` 方法
4. 如果某个钩子返回错误，可以选择中断或继续执行

```rust
async fn execute_hooks(hooks: &[Box<dyn Hook>], ctx: &EventContext) {
    // 按优先级排序
    let mut sorted_hooks: Vec<_> = hooks.iter().collect();
    sorted_hooks.sort_by_key(|h| h.rank());

    // 依次执行
    for hook in sorted_hooks {
        match hook.run(Some(ctx)).await {
            Ok(_) => println!("钩子 {} 执行成功", hook.name()),
            Err(e) => {
                eprintln!("钩子 {} 执行失败: {}", hook.name(), e);
                // 可以选择中断或继续
                break;
            }
        }
    }
}
```

## 特性标志

### registry

启用钩子注册表功能。

```toml
[dependencies]
puniyu_hook = { version = "*", features = ["registry"] }
```

启用后可以使用 `HookRegistry` 来管理多个钩子实例。

## 最佳实践

### 1. 合理设置优先级

```rust
// 权限检查应该最先执行
impl Hook for AuthHook {
    fn rank(&self) -> u32 { 10 }
}

// 业务逻辑在中间执行
impl Hook for BusinessHook {
    fn rank(&self) -> u32 { 100 }
}

// 日志记录最后执行
impl Hook for LogHook {
    fn rank(&self) -> u32 { 200 }
}
```

### 2. 错误处理

```rust
async fn run(&self, ctx: Option<&EventContext>) -> puniyu_error::Result {
    ctx.ok_or_else(|| puniyu_error::Error::msg("上下文为空"))?;

    // 处理逻辑

    Ok(())
}
```

### 3. 避免阻塞

```rust
async fn run(&self, ctx: Option<&EventContext>) -> puniyu_error::Result {
    // 使用 tokio::spawn 避免阻塞
    tokio::spawn(async move {
        // 耗时操作
    });

    Ok(())
}
```

### 4. 状态管理

```rust
struct StatefulHook {
    state: std::sync::Arc<std::sync::Mutex<State>>,
}

#[async_trait]
impl Hook for StatefulHook {
    async fn run(&self, ctx: Option<&EventContext>) -> puniyu_error::Result {
        let mut state = self.state.lock().unwrap();
        // 使用状态
        Ok(())
    }
}
```

## 文档

完整的 API 文档请访问 [docs.rs](https://docs.rs/puniyu_hook)。

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。

## 相关链接

- [GitHub 仓库](https://github.com/puniyu/puniyu)
- [Puniyu 框架](https://github.com/puniyu/puniyu)
- [puniyu_context](https://docs.rs/puniyu_context) - 上下文管理库
- [puniyu_event](https://docs.rs/puniyu_event) - 事件类型库
