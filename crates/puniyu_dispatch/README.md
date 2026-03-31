# puniyu_dispatch

puniyu 事件分发库，提供全局事件发射器用于异步事件处理。

## 特性

- ⚡ **异步分发**: 基于 tokio 异步运行时，支持高并发事件处理
- 📊 **优先级调度**: 处理器按 `priority()` 升序依次执行
- 🌐 **全局单例**: 通过 `EventEmitter` 在任意位置发送事件
- 🛡️ **容错处理**: 单个处理器出错不影响后续处理器执行

## 示例

```rust
use puniyu_dispatch::EventEmitter;
use puniyu_event::Event;

#[tokio::main]
async fn main() {
    // 启动事件发射器
    EventEmitter::run().expect("启动失败");

    // 分发事件到所有已注册处理器
    let event = Event::Message(message_event);
    EventEmitter::emit(event).await.unwrap();

    // 停止事件发射器
    EventEmitter::stop();
}
```

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。
