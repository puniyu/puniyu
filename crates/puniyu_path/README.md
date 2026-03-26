# puniyu_path

路径管理库，统一提供应用运行所需的目录路径。

## 特性

- 📁 **基础路径**: `app_cwd_dir`、`app_dir`、`config_dir`、`data_dir` 等
- 🧩 **模块路径**: 提供 `plugin::*` 与 `adapter::*` 子目录路径
- 🔗 **统一规则**: 所有路径基于 `puniyu_common::app` 当前应用信息生成

## 示例

```rust
use puniyu_path::{adapter, config_dir, plugin};

let config = config_dir();
let plugin_config = plugin::config_dir();
let adapter_data = adapter::data_dir();

let _ = (config, plugin_config, adapter_data);
```

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。
