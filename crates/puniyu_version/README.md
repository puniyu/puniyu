# puniyu_version

轻量版本号类型，统一表示 `major.minor.patch`。

## 特性

- 🔢 **三段版本**: 仅表示 `major.minor.patch`
- ✍️ **易用接口**: 支持构造、显示、解析
- 🔄 **类型互转**: 支持与 `semver` 互转
- 🧩 **序列化**: 支持 `serde` 序列化与反序列化

## 示例

```rust
use std::str::FromStr;
use puniyu_version::Version;

let v = Version::new(1, 2, 3);
assert_eq!(v.to_string(), "1.2.3");

let parsed = Version::from_str("1.2.3-beta.1+build.9").unwrap();
assert_eq!(parsed, Version::new(1, 2, 3));
```

说明：预发布和构建元数据会被忽略，只保留核心三段版本。

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。
