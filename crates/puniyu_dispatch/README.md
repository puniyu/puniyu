# puniyu_dispatch

事件分发和处理库，提供全局事件总线用于异步事件分发。

## 概述

`puniyu_dispatch` 是 Puniyu 框架的事件分发核心，提供了一个高性能的异步事件总线。该库负责将事件从产生者分发到所有注册的处理器，支持跨线程事件传递和优先级调度。

## 功能特性

- 🚀 **高性能** - 基于 tokio 异步运行时，支持高并发事件处理
- 🔄 **异步分发** - 使用异步通道进行事件传递，避免阻塞
- 📊 **优先级调度** - 处理器按优先级（priority）顺序执行
- 🌐 **跨线程** - 使用 Arc 包装事件，支持跨线程共享
- 🛡️ **优雅关闭** - 提供关闭机制，确保事件处理完成
- 🔌 **全局单例** - 全局事件总线，便于在任何地方发送事件

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
puniyu_dispatch = "*"
```

## 快速开始

### 1. 初始化事件总线

```rust
use puniyu_dispatch::{EventBus, setup_event_bus};

#[tokio::main]
async fn main() {
    // 创建事件总线
    let event_bus = EventBus::new();

    // 设置为全局实例
    setup_event_bus(event_bus);

    // 启动事件处理循环
    let event_bus = puniyu_dispatch::get_event_bus();
    let handle = event_bus.run();

    // 等待事件总线关闭
    handle.await.unwrap();
}
```

### 2. 发送事件

```rust
use std::sync::Arc;
use puniyu_dispatch::get_event_bus;
use puniyu_event::Event;

// 获取事件总线
let event_bus = get_event_bus();

// 创建事件（必须用 Arc 包装）
let event = Arc::new(my_event);

// 发送事件
event_bus.send_event(event);
```

### 3. 关闭事件总线

```rust
use puniyu_dispatch::get_event_bus;

let event_bus = get_event_bus();
event_bus.shutdown();
```

## 核心概念

### EventBus - 事件总线

事件总线是事件分发的核心，负责：

- 接收事件
- 分发事件到处理器
- 管理事件处理生命周期

```rust
// 创建默认容量（5000）的事件总线
let event_bus = EventBus::new();

// 创建自定义容量的事件总线
let event_bus = EventBus::with_capacity(10000);
```

### 事件发送

事件必须使用 `Arc` 包装，以支持跨线程共享：

```rust
use std::sync::Arc;

let event = Arc::new(Event::Message(message_event));
event_bus.send_event(event);
```

### 事件处理

事件会被分发到所有注册的处理器，处理器按优先级（priority）排序后依次执行：

```rust
// 处理器通过 puniyu_handler 库注册
use puniyu_handler::{Handler, HandlerRegistry};

// 注册处理器
HandlerRegistry::register(my_handler)?;
```

## 工作流程

```
┌─────────────┐
│  发送事件    │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  事件总线    │
│  (EventBus) │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  事件通道    │
│  (mpsc)     │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  事件循环    │
│  (run)      │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  分发事件    │
│  (dispatch) │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  处理器 1    │
│  (priority: 0)  │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  处理器 2    │
│  (priority: 10) │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  处理器 N    │
│  (priority: 99) │
└─────────────┘
```

## API 文档

### EventBus

| 方法              | 说明                   | 返回值           |
| ----------------- | ---------------------- | ---------------- |
| `new()`           | 创建默认容量的事件总线 | `EventBus`       |
| `with_capacity()` | 创建指定容量的事件总线 | `EventBus`       |
| `run()`           | 启动事件处理循环       | `JoinHandle<()>` |
| `send_event()`    | 发送事件               | -                |
| `shutdown()`      | 关闭事件总线           | -                |
| `sender()`        | 获取事件发送器         | `EventSender`    |
| `is_running()`    | 检查是否正在运行       | `bool`           |

### 全局函数

| 函数              | 说明             | 参数       |
| ----------------- | ---------------- | ---------- |
| `setup_event_bus` | 设置全局事件总线 | `EventBus` |
| `get_event_bus`   | 获取全局事件总线 | -          |

## 完整示例

```rust
use puniyu_dispatch::{EventBus, setup_event_bus, get_event_bus};
use puniyu_event::Event;
use puniyu_handler::{Handler, HandlerRegistry};
use std::sync::Arc;

// 定义事件处理器
struct MyHandler;

#[async_trait::async_trait]
impl Handler for MyHandler {
    fn name(&self) -> &str {
        "my_handler"
    }

    fn priority(&self) -> u8 {
        10
    }

    async fn handle(&self, ctx: &EventContext) -> Result<()> {
        println!("处理事件: {:?}", ctx.event());
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // 1. 注册处理器
    HandlerRegistry::register(Arc::new(MyHandler))?;

    // 2. 初始化事件总线
    let event_bus = EventBus::new();
    setup_event_bus(event_bus);

    // 3. 启动事件处理循环
    let event_bus = get_event_bus();
    let handle = event_bus.run();

    // 4. 发送事件
    let event = Arc::new(Event::Message(message_event));
    event_bus.send_event(event);

    // 5. 等待一段时间后关闭
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    event_bus.shutdown();

    // 6. 等待事件总线完全关闭
    handle.await?;

    Ok(())
}
```

## 性能优化

### 1. 调整通道容量

根据事件产生速率调整通道容量：

```rust
// 高频事件场景
let event_bus = EventBus::with_capacity(10000);

// 低频事件场景
let event_bus = EventBus::with_capacity(1000);
```

### 2. 处理器优先级

合理设置处理器优先级，确保重要处理器优先执行：

```rust
impl Handler for CriticalHandler {
    fn priority(&self) -> u8 {
        0  // 最高优先级
    }
}

impl Handler for NormalHandler {
    fn priority(&self) -> u8 {
        50  // 中等优先级
    }
}

impl Handler for LowPriorityHandler {
    fn priority(&self) -> u8 {
        99  // 最低优先级
    }
}
```

### 3. 避免阻塞操作

处理器中避免执行阻塞操作，使用异步 API：

```rust
async fn handle(&self, ctx: &EventContext) -> Result<()> {
    // ✅ 好的做法：使用异步 API
    let data = fetch_data_async().await?;

    // ❌ 不好的做法：阻塞操作
    // let data = fetch_data_blocking();

    Ok(())
}
```

## 注意事项

### 生命周期要求

事件必须具有 `'static` 生命周期：

```rust
// ✅ 正确：使用 'static 生命周期
let event: Event<'static> = Event::Message(message);
let event = Arc::new(event);

// ❌ 错误：非 'static 生命周期
let temp_data = String::from("temp");
let event = Event::Custom(&temp_data);  // 编译错误
```

### 单例限制

事件总线只能初始化一次：

```rust
// ✅ 正确：只初始化一次
setup_event_bus(EventBus::new());

// ❌ 错误：重复初始化会 panic
setup_event_bus(EventBus::new());  // panic!
```

### 关闭顺序

确保在应用退出前正确关闭事件总线：

```rust
// 1. 停止发送新事件
// 2. 关闭事件总线
event_bus.shutdown();
// 3. 等待处理完成
handle.await?;
```

## 最佳实践

### 1. 错误处理

在处理器中正确处理错误：

```rust
async fn handle(&self, ctx: &EventContext) -> Result<()> {
    match process_event(ctx).await {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("处理事件失败: {}", e);
            Err(e)
        }
    }
}
```

### 2. 日志记录

使用 `puniyu_logger` 记录关键事件：

```rust
use puniyu_logger::{info, warn, error};

info!("事件总线已启动");
warn!("事件通道接近满载");
error!("事件处理失败: {}", e);
```

### 3. 监控和调试

检查事件总线状态：

```rust
if event_bus.is_running() {
    println!("事件总线正在运行");
} else {
    println!("事件总线未启动");
}
```

## 故障排查

### 事件未被处理

1. 检查处理器是否已注册
2. 检查事件总线是否已启动（调用 `run()`）
3. 检查处理器的 `handle` 方法是否返回错误

### 事件丢失

1. 检查通道容量是否足够
2. 检查是否有处理器阻塞
3. 查看日志中的警告信息

### 内存占用高

1. 减小通道容量
2. 优化处理器性能
3. 及时释放事件引用

## 文档

完整的 API 文档请访问 [docs.rs](https://docs.rs/puniyu_dispatch)。

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。

## 相关链接

- [GitHub 仓库](https://github.com/puniyu/puniyu)
- [Puniyu 框架](https://github.com/puniyu/puniyu)
- [puniyu_event](https://docs.rs/puniyu_event) - 事件定义库
- [puniyu_handler](https://docs.rs/puniyu_handler) - 处理器库
- [puniyu_context](https://docs.rs/puniyu_context) - 上下文库
