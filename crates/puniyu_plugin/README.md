# puniyu_plugin_core

插件协议层，定义 Puniyu 中插件的核心抽象和可扩展能力。

## 特性

- 提供 `Plugin` trait，统一定义 `on_start`、`on_load`、`on_unload`、`on_stop` 生命周期回调
- 支持插件元信息（名称、版本、描述、作者）
- 支持稳定的生命周期优先级
- 通过 `PluginContext` 发布和查询类型安全的共享能力

## 快速开始

```rust
use async_trait::async_trait;
use puniyu_context::PluginContext;
use puniyu_error::AnyError;
use puniyu_plugin_core::Plugin;
use semver::Version;

struct MyPlugin;

#[async_trait]
impl Plugin for MyPlugin {
    fn name(&self) -> &str { "my-plugin" }
    fn version(&self) -> Version { Version::new(1, 0, 0) }
    fn description(&self) -> Option<&str> { Some("my plugin") }
    async fn on_start(&self, _ctx: &PluginContext) -> AnyError { Ok(()) }
    async fn on_load(&self, _ctx: &PluginContext) -> AnyError { Ok(()) }
    async fn on_unload(&self, _ctx: &PluginContext) -> AnyError { Ok(()) }
    async fn on_stop(&self, _ctx: &PluginContext) -> AnyError { Ok(()) }
}
```
