<div align="center">

# puniyu_macros

**过程宏库，提供插件、适配器、命令和任务等声明式入口**

</div>

<div align="center">

[![crates.io](https://img.shields.io/crates/v/puniyu_macros?color=%23FDD835&label=puniyu_macros&style=for-the-badge)](https://crates.io/crates/puniyu_macros)
[![License](https://img.shields.io/github/license/puniyu/puniyu?style=for-the-badge)](../../LICENSE)

</div>

---

## 概述

`puniyu_macros` 是 puniyu 生态的过程宏库，为插件和适配器开发提供声明式宏。它负责在编译期生成 trait 实现、inventory 注册、配置读写等样板代码，让开发者专注于业务逻辑。

## 特性

- 插件侧宏：`#[plugin]`、`#[command]`、`#[arg]`、`#[task]`、`#[server]`、`#[derive(PluginConfig)]`
- 适配器侧宏：`#[adapter]`、`#[server]`、`#[derive(AdapterConfig)]`
- 生命周期钩子：`#[on_load]`、`#[on_unload]`
- 编译期校验：函数签名、参数类型、cron 表达式、权限值合法性

## 快速开始

### 添加依赖

```toml
[dependencies]
puniyu_macros = "0.8"
```

通常不需要直接依赖此 crate，使用 `puniyu_plugin` 或 `puniyu_adapter` 即可获得全部宏。

## 宏参考

### 插件侧

#### `#[plugin]`

声明插件结构体或 impl 块，自动生成 Plugin trait 实现并注册到 inventory。

```rust
#[plugin(desc = "插件描述", prefix = "!", config = MyConfig)]
struct MyPlugin;
```

| 属性 | 说明 |
|---|---|
| `desc` | 插件描述文本 |
| `prefix` | 插件命令前缀，会与全局前缀组合 |
| `config` | 配置结构体类型，需实现 `#[derive(PluginConfig)]` |

可在 impl 块上使用，此时不支持 `desc`/`prefix`/`config` 属性（需在 struct 上声明）：

```rust
#[plugin]
impl MyPlugin {
    #[on_load]
    async fn on_load(&self) -> puniyu_plugin::result::Result { Ok(()) }

    #[on_unload]
    async fn on_unload(&self) -> puniyu_plugin::result::Result { Ok(()) }

    #[server]
    async fn my_server(&self) { /* HTTP 服务 */ }
}
```

#### `#[command]`

声明命令处理函数，自动生成 Command trait 实现并注册。

```rust
#[command(name = "echo", desc = "复读消息", priority = 100, permission = "admin")]
#[arg(name = "text", desc = "要复读的内容", required = true)]
async fn echo(ctx: &MessageContext) -> puniyu_plugin::result::Result<CommandAction> {
    // ...
    Ok(CommandAction::Stop)
}
```

| 属性 | 说明 |
|---|---|
| `name` | 命令名称，默认为函数名的 snake_case |
| `desc` | 命令描述 |
| `priority` | 执行优先级，数字越小越先执行，默认 500 |
| `permission` | 所需权限：`"all"`（默认）/ `"admin"` / `"owner"` / `"master"` |
| `alias` | 命令别名列表，如 `alias = ["e", "repeat"]` |

#### `#[arg]`

为命令补充参数描述，可叠加多个。

```rust
#[arg(name = "text", desc = "输入文本", required = true)]
#[arg(name = "count", desc = "重复次数", arg_type = "number")]
```

| 属性 | 说明 |
|---|---|
| `name` | 参数名称 |
| `desc` | 参数描述 |
| `required` | 是否必填，默认 `false` |
| `arg_type` | 参数类型描述 |
| `mode` | 参数模式描述 |

#### `#[task]`

声明定时任务函数，自动生成 Task trait 实现并注册。函数必须是 `async fn`，无参数，返回 `puniyu_plugin::result::Result`。

```rust
#[task(name = "daily_check", cron = "0 8 * * *")]
async fn daily_check() -> puniyu_plugin::result::Result {
    Ok(())
}
```

| 属性 | 说明 |
|---|---|
| `name` | 任务名称，默认为函数名的 snake_case |
| `cron` | Cron 表达式（必填），编译期校验格式合法性 |

#### `#[derive(PluginConfig)]`

为插件配置结构体派生 `puniyu_core::config::Config` trait，自动生成文件路径和序列化逻辑。

```rust
#[derive(Debug, Clone, Default, Deserialize, Serialize, PluginConfig)]
struct MyConfig {
    pub api_key: String,
}
```

派生后会根据结构体名自动生成 `config/{plugin_name}/{config_name}.toml` 的读取路径。

### 适配器侧

#### `#[adapter]`

声明适配器结构体或 impl 块，用法与 `#[plugin]` 类似。

```rust
#[adapter(config = MyConfig)]
struct MyAdapter;

#[adapter]
impl MyAdapter {
    #[on_load]
    async fn on_load(&self) -> puniyu_adapter::result::Result { Ok(()) }

    #[on_unload]
    async fn on_unload(&self) -> puniyu_adapter::result::Result { Ok(()) }
}
```

| 属性 | 说明 |
|---|---|
| `config` | 配置结构体类型，需实现 `#[derive(AdapterConfig)]` |

#### `#[derive(AdapterConfig)]`

为适配器配置结构体派生 Config trait，用法与 `#[derive(PluginConfig)]` 相同。

### 通用

#### `#[on_load]` / `#[on_unload]`

标记生命周期钩子函数，必须是 `async fn`，无参数，返回对应的 `Result` 类型。

#### `#[server]`

标记服务函数，用于声明 HTTP 服务端点。

## 编译期校验

宏在编译期会校验以下内容：

- 被标注函数必须是 `async fn`
- 生命周期函数不能有参数
- 返回类型必须匹配指定的 `Result` 类型
- `cron` 表达式格式必须合法
- `permission` 值必须是 `all` / `admin` / `owner` / `master` 之一
- `#[on_load]` / `#[on_unload]` 在同一个 impl 块中不能重复

## 许可协议

与 puniyu 项目一致，采用 [LGPL-3.0](../../LICENSE) 协议。
