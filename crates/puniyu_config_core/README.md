# puniyu_config_core

配置核心 trait 和注册表。

## 使用

```rust
use puniyu_config_core::{Config, ConfigRegistry};
use std::path::Path;
use toml::Value;

// 实现 Config trait
struct MyConfig;

impl Config for MyConfig {
    fn name(&self) -> &'static str {
        "my_config"
    }

    fn path(&self) -> &'static Path {
        Path::new("config/my_config.toml")
    }

    fn config(&self) -> Value {
        toml::toml! {
            enabled = true
            timeout = 30
        }
    }
}

// 注册配置
let config = Arc::new(MyConfig);
let id = ConfigRegistry::register(config)?;

// 查询配置
let value = ConfigRegistry::get(id);
let value = ConfigRegistry::get(Path::new("config/my_config.toml"));

// 更新配置
ConfigRegistry::update(id, new_value)?;
```

## 特性

- `Config` trait - 配置接口定义
- `ConfigRegistry` - 全局配置注册表（需启用 `registry` feature）
- 支持通过索引或路径查询配置
- 线程安全的配置存储

详细文档见 [docs.rs](https://docs.rs/puniyu_config_core)

## 许可证

[LGPL-3.0](../../LICENSE)
