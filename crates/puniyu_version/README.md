# puniyu_version

版本号定义库，提供语义化版本号的类型系统。

## 概述

`puniyu_version` 提供了符合语义化版本规范（Semantic Versioning）的版本号结构定义，用于管理和比较版本信息。版本号格式为 `major.minor.patch`。

## 特性

- 🎯 **语义化版本** - 符合 SemVer 规范的版本号管理
- 🔄 **类型转换** - 支持字符串和版本号之间的相互转换
- 📦 **序列化支持** - 内置 serde 支持
- 🎨 **简洁 API** - 直观易用的接口设计

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
puniyu_version = "*"
```

## 快速开始

### 创建版本号

```rust
use puniyu_version::Version;

// 手动创建
let version = Version {
    major: 1,
    minor: 2,
    patch: 3,
};

// 从字符串创建
let version: Version = "1.2.3".into();

// 使用默认版本（0.0.1）
let version = Version::default();
```

### 版本号显示

```rust
use puniyu_version::Version;

let version = Version {
    major: 1,
    minor: 2,
    patch: 3,
};

println!("Version: {}", version); // 输出: Version: 1.2.3
```

### 版本号解析

```rust
use puniyu_version::Version;
use std::str::FromStr;

// 从字符串解析
let version = Version::from_str("1.2.3").unwrap();
assert_eq!(version.major, 1);
assert_eq!(version.minor, 2);
assert_eq!(version.patch, 3);

// 解析失败返回默认版本（0.0.1）
let version = Version::from_str("invalid").unwrap();
assert_eq!(version, Version::default());
```

## Version 结构

| 字段    | 类型  | 说明                                 |
| ------- | ----- | ------------------------------------ |
| `major` | `u16` | 主版本号，不兼容的 API 修改时递增    |
| `minor` | `u16` | 次版本号，向下兼容的功能性新增时递增 |
| `patch` | `u16` | 补丁版本号，向下兼容的问题修正时递增 |

## 语义化版本规范

版本号格式：`major.minor.patch`

- **主版本号（major）**：当你做了不兼容的 API 修改
- **次版本号（minor）**：当你做了向下兼容的功能性新增
- **补丁版本号（patch）**：当你做了向下兼容的问题修正

## 类型转换

### 从字符串创建

```rust
use puniyu_version::Version;

// 从 &str
let version: Version = "1.2.3".into();

// 从 String
let s = String::from("1.2.3");
let version: Version = s.into();
```

### 转换为字符串

```rust
use puniyu_version::Version;

let version = Version {
    major: 1,
    minor: 2,
    patch: 3,
};

// 使用 Display trait
let s = version.to_string();
assert_eq!(s, "1.2.3");

// 使用 Into trait
let s: String = version.into();
assert_eq!(s, "1.2.3");
```

## 序列化和反序列化

`Version` 支持 serde 序列化和反序列化：

```rust
use puniyu_version::Version;

let version = Version {
    major: 1,
    minor: 2,
    patch: 3,
};

// 序列化为 JSON
let json = serde_json::to_string(&version).unwrap();

// 从 JSON 反序列化
let deserialized: Version = serde_json::from_str(&json).unwrap();

assert_eq!(version, deserialized);
```

## 比较和克隆

```rust
use puniyu_version::Version;

let version1 = Version { major: 1, minor: 2, patch: 3 };
let version2 = Version { major: 1, minor: 2, patch: 3 };
let version3 = Version { major: 2, minor: 0, patch: 0 };

// 相等性比较
assert_eq!(version1, version2);
assert_ne!(version1, version3);

// 克隆
let version4 = version1.clone();
assert_eq!(version1, version4);
```

## 默认版本

默认版本号为 `0.0.1`：

```rust
use puniyu_version::Version;

let version = Version::default();
assert_eq!(version.major, 0);
assert_eq!(version.minor, 0);
assert_eq!(version.patch, 1);
```

## 解析行为

- 正确格式：`"1.2.3"` → `Version { major: 1, minor: 2, patch: 3 }`
- 格式错误：`"invalid"` → `Version::default()` (0.0.1)
- 部分无效：`"1.a.3"` → `Version { major: 1, minor: 0, patch: 3 }`
- 段数错误：`"1.2"` → `Version::default()` (0.0.1)

## 文档

完整的 API 文档请访问 [docs.rs](https://docs.rs/puniyu_version)。

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。

## 相关链接

- [GitHub 仓库](https://github.com/puniyu/puniyu)
- [Puniyu 框架](https://github.com/puniyu/puniyu)
- [语义化版本规范](https://semver.org/lang/zh-CN/)
