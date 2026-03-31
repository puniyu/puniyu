# puniyu_error

错误类型库，提供配置与注册表场景的统一错误定义。

## 特性

- 🧱 **统一错误模型**: 提供 `config::Error` 与 `registry::Error`
- 🔧 **按需启用**: 支持 `config`、`registry`、`full` feature
- 🔄 **统一结果类型**: 提供通用 `Result<T>` 别名
- 📝 **清晰错误信息**: 基于 `thiserror` 输出可读错误消息

## 示例

```rust
use puniyu_error::{registry, Result};

fn check(name: &str, exists: bool) -> Result {
    if exists {
        Err(Box::new(registry::Error::Exists(name.to_string())))
    } else {
        Ok(())
    }
}
```

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。
