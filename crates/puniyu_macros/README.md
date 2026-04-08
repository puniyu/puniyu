# puniyu_macros

`puniyu_macros` 是 puniyu 生态的过程宏库，用于声明插件、命令、任务、配置和适配器相关入口。

## 宏列表

### 插件侧宏

- `#[plugin]`：声明插件入口函数
- `#[plugin_config]`：声明插件配置结构体
- `#[plugin_hook]`：声明插件钩子函数
- `#[command]`：声明命令处理函数
- `#[arg]`：为命令补充参数描述
- `#[task]`：声明定时任务函数

### 适配器侧宏

- `#[adapter]`：声明适配器入口函数
- `#[adapter_config]`：声明适配器配置结构体
- `#[adapter_hook]`：声明适配器钩子函数

## 使用说明

### `#[plugin]`

用于声明插件入口函数。

要求：
- 函数必须是 `async`
- 当函数体非空时，返回类型必须为 `puniyu_plugin::Result`
- 当函数体为空时，可以省略返回值类型

最小示例：

```rust
use puniyu_plugin::prelude::*;

#[plugin]
async fn __main() {}
```

完整示例：

```rust
use puniyu_plugin::prelude::*;

#[plugin(desc = "基础功能插件", prefix = "basic")]
async fn __main() -> puniyu_plugin::Result {
	log::info!("plugin init");
	Ok(())
}
```

---

### `#[command]`

用于声明命令处理函数。

常用参数：
- `name`：命令名称，必填
- `desc`：命令描述，可选
- `priority`：优先级，可选，默认 `500`
- `alias`：别名列表，可选
- `permission`：权限，可选，支持 `"all"` 和 `"admin"`

要求：
- 函数必须是 `async`
- 必须且只能接收一个参数：`&MessageContext<'_>`
- 返回类型必须为 `puniyu_plugin::Result<CommandAction>`

基础示例：

```rust
use puniyu_plugin::prelude::*;

#[command(name = "state", desc = "查看运行状态", alias = ["status"], priority = 100)]
async fn state(ctx: &MessageContext<'_>) -> puniyu_plugin::Result<CommandAction> {
	ctx.reply(message!(segment!(text, "running"))).await?;
	Ok(CommandAction::Done)
}
```

带参数示例：

```rust
use puniyu_plugin::prelude::*;

#[command(name = "echo", desc = "回显文本", permission = "all")]
#[arg(name = "message", desc = "要发送的文本", required = true)]
#[arg(name = "times", type = "integer", mode = "optional", desc = "重复次数")]
async fn echo(ctx: &MessageContext<'_>) -> puniyu_plugin::Result<CommandAction> {
	ctx.reply(message!(segment!(text, "echo"))).await?;
	Ok(CommandAction::Done)
}
```

---

### `#[arg]`

用于为 `#[command]` 生成命令参数描述。

常用参数：
- `name`：参数名，必填
- `desc`：参数说明，可选
- `type`：参数类型，可选，支持 `string`、`integer`、`boolean`
- `mode`：参数模式，可选，支持 `positional`、`optional`
- `required`：是否必填，可选

完整示例：

```rust
use puniyu_plugin::prelude::*;

#[command(name = "echo")]
#[arg(name = "message", desc = "消息内容", required = true)]
#[arg(name = "times", type = "integer", mode = "optional", desc = "重复次数")]
#[arg(name = "silent", type = "boolean", mode = "optional", desc = "是否静默执行")]
async fn echo(ctx: &MessageContext<'_>) -> puniyu_plugin::Result<CommandAction> {
	Ok(CommandAction::Done)
}
```

---

### `#[task]`

用于声明定时任务函数。

常用参数：
- `cron`：cron 表达式，必填
- `name`：任务名，可选，默认使用函数名

要求：
- 函数必须是 `async`
- 函数不接收参数
- 返回类型必须为 `puniyu_plugin::Result`
- `cron` 会在编译时校验

基础示例：

```rust
use puniyu_plugin::prelude::*;

#[task(cron = "0 0 * * *", name = "daily_job")]
async fn daily_job() -> puniyu_plugin::Result {
	Ok(())
}
```

多个任务示例：

```rust
use puniyu_plugin::prelude::*;

#[task(cron = "*/5 * * * *", name = "health_check")]
async fn health_check() -> puniyu_plugin::Result {
	Ok(())
}

#[task(cron = "0 0 * * 1", name = "weekly_cleanup")]
async fn weekly_cleanup() -> puniyu_plugin::Result {
	Ok(())
}
```

---

### `#[plugin_config]`

用于声明插件配置结构体，并注册到插件配置列表中。

要求：
- 通常用于可序列化且实现了 `Default` 的结构体
- 默认配置文件名取结构体名的 snake_case，也可以通过 `name` 指定

完整示例：

```rust
use serde::{Deserialize, Serialize};
use puniyu_macros::plugin_config;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[plugin_config(name = "basic")]
pub struct BasicConfig {
	pub enabled: bool,
	pub prefix: String,
}
```

---

### `#[plugin_hook]`

用于声明插件钩子函数。

常用参数：
- `name`：钩子名称，可选
- `hook_type`：钩子类型，可选，如 `"event.message"`、`"status.start"`
- `priority`：优先级，可选，默认 `500`

要求：
- 函数必须是 `async`
- 必须且只能接收一个引用参数，对应 `HookType`
- 返回类型必须为 `puniyu_plugin::Result`

基础示例：

```rust
use puniyu_plugin::hook::HookType;
use puniyu_macros::plugin_hook;

#[plugin_hook(name = "on_message", hook_type = "event.message", priority = 100)]
async fn on_message(event: &HookType) -> puniyu_plugin::Result {
	let _ = event;
	Ok(())
}
```

状态钩子示例：

```rust
use puniyu_plugin::hook::HookType;
use puniyu_macros::plugin_hook;

#[plugin_hook(name = "on_start", hook_type = "status.start")]
async fn on_start(event: &HookType) -> puniyu_plugin::Result {
	let _ = event;
	Ok(())
}
```

`hook_type` 支持的常见值：
- `event`
- `event.message`
- `event.all`
- `status`
- `status.start`
- `status.stop`

---

### `#[adapter]`

用于声明适配器入口函数。

常用参数：
- `info`：返回适配器信息的函数，必填
- `runtime`：返回适配器运行时的函数，必填

要求：
- 函数必须是 `async`
- 当函数体非空时，返回类型必须为 `puniyu_adapter::Result`
- 当函数体为空时，可以省略返回值类型

完整示例：

```rust
use puniyu_adapter::types::*;

mod api {
	use puniyu_adapter::runtime::AdapterRuntime;
	pub fn runtime() -> AdapterRuntime {
		unimplemented!()
	}
}

fn info() -> AdapterInfo {
	adapter_info!(
		name: env!("CARGO_PKG_NAME"),
		version: pkg_version!(),
		platform: AdapterPlatform::Other,
		standard: AdapterStandard::Other,
		protocol: AdapterProtocol::Console,
		communication: AdapterCommunication::Other
	)
}

#[adapter(info = info, api = api::api)]
async fn main() -> puniyu_adapter::Result {
	Ok(())
}
```

---

### `#[adapter_config]`

用于声明适配器配置结构体，并注册到适配器配置列表中。

要求：
- 通常用于可序列化且实现了 `Default` 的结构体
- 默认配置文件名取结构体名的 snake_case，也可以通过 `name` 指定

完整示例：

```rust
use serde::{Deserialize, Serialize};
use puniyu_macros::adapter_config;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[adapter_config(name = "console")]
pub struct ConsoleConfig {
	pub enabled: bool,
	pub prompt: String,
}
```

如果不指定 `name`，默认会使用结构体名的 snake_case 作为配置名和文件名。

---

### `#[adapter_hook]`

用于声明适配器钩子函数。

常用参数：
- `name`：钩子名称，可选
- `hook_type`：钩子类型，可选，如 `"event.message"`、`"status.start"`
- `priority`：优先级，可选，默认 `500`

要求：
- 函数必须是 `async`
- 必须且只能接收一个引用参数，对应 `HookType`
- 返回类型必须为 `puniyu_adapter::Result`

基础示例：

```rust
use puniyu_adapter::hook::HookType;
use puniyu_macros::adapter_hook;

#[adapter_hook(name = "on_start", hook_type = "status.start", priority = 100)]
async fn on_start(event: &HookType) -> puniyu_adapter::Result {
	let _ = event;
	Ok(())
}
```

事件钩子示例：

```rust
use puniyu_adapter::hook::HookType;
use puniyu_macros::adapter_hook;

#[adapter_hook(name = "on_event_message", hook_type = "event.message")]
async fn on_event_message(event: &HookType) -> puniyu_adapter::Result {
	let _ = event;
	Ok(())
}
```

`hook_type` 支持的常见值：
- `event`
- `event.message`
- `event.all`
- `status`
- `status.start`
- `status.stop`

## 参考示例

- 插件入口：[plugins/puniyu_plugin_basic/src/lib.rs](../../plugins/puniyu_plugin_basic/src/lib.rs)
- 命令示例：[plugins/puniyu_plugin_basic/src/command/state.rs](../../plugins/puniyu_plugin_basic/src/command/state.rs)
- 适配器入口：[adapters/puniyu_adapter_console/src/lib.rs](../../adapters/puniyu_adapter_console/src/lib.rs)

## 许可证

本项目采用 [LGPL](../../LICENSE) 许可证。
