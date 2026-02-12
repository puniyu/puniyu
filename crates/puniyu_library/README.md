# puniyu_library

动态库管理模块，提供动态库的加载、卸载和管理功能。

## 概述

`puniyu_library` 基于 `libloading` 实现动态库的加载、获取、卸载和重载功能，使用全局注册表统一管理所有已加载的动态库。该库提供了类型安全、线程安全的 API，适用于插件系统和动态模块加载场景。

## 特性

- 🎯 **类型安全** - 使用 Rust 类型系统确保动态库操作的安全性
- 🔧 **全局管理** - 提供全局注册表统一管理所有动态库
- 🔄 **热重载** - 支持动态库的热重载功能
- 📦 **线程安全** - 使用 `Mutex` 保证多线程环境下的安全性
- 🎨 **易于使用** - 提供简洁直观的 API 接口
- ⚡ **引用计数** - 自动管理库的生命周期，防止意外卸载

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
puniyu_library = "*"
```

## 快速开始

### 加载动态库

```rust
use puniyu_library::LibraryRegistry;
use std::path::Path;

// 从路径加载
LibraryRegistry::load(Path::new("plugin.dll"))?;

// 从字符串路径加载
LibraryRegistry::load_from_path("plugin.dll")?;

// 强制加载（如果已存在则替换）
LibraryRegistry::load_or_replace(Path::new("plugin.dll"))?;
```

### 获取和使用动态库

```rust
use puniyu_library::LibraryRegistry;

// 获取库信息
if let Some(lib) = LibraryRegistry::get("plugin.dll") {
    println!("库名称: {}", lib.name());
    println!("库路径: {:?}", lib.path());

    // 使用库中的函数
    unsafe {
        let func: libloading::Symbol<fn() -> i32> =
            lib.library().get(b"my_function")?;
        let result = func();
        println!("结果: {}", result);
    }
}
```

### 检查和列出库

```rust
use puniyu_library::LibraryRegistry;

// 检查库是否存在
if LibraryRegistry::contains("plugin.dll") {
    println!("库已加载");
}

// 列出所有已加载的库
let libraries = LibraryRegistry::list();
for name in libraries {
    println!("已加载: {}", name);
}

// 获取库数量
let count = LibraryRegistry::count();
println!("共加载了 {} 个库", count);
```

### 卸载和重载

```rust
use puniyu_library::LibraryRegistry;

// 卸载库
let path = LibraryRegistry::unload("plugin.dll")?;
println!("已卸载: {:?}", path);

// 强制卸载（即使正在使用）
let path = LibraryRegistry::unload_force("plugin.dll")?;

// 重新加载库（热重载）
LibraryRegistry::reload("plugin.dll")?;

// 清空所有库
LibraryRegistry::clear()?;

// 强制清空所有库
LibraryRegistry::clear_force();
```

## 核心组件

### LibraryInfo

库信息结构体，包含动态库的元数据和句柄。

| 字段      | 类型           | 说明                   |
| --------- | -------------- | ---------------------- |
| `name`    | `String`       | 库名称（从文件名提取） |
| `path`    | `PathBuf`      | 库文件的完整路径       |
| `library` | `Arc<Library>` | 动态库句柄的 Arc 包装  |

#### 方法

```rust
// 创建库信息
let info = LibraryInfo::new(PathBuf::from("plugin.dll"))?;

// 获取库名称
let name = info.name();

// 获取库路径
let path = info.path();

// 获取库句柄
let library = info.library();
```

### LibraryRegistry

全局库注册表，提供静态方法管理动态库。

| 方法              | 说明                     | 返回类型                   |
| ----------------- | ------------------------ | -------------------------- |
| `load`            | 加载动态库               | `Result<(), Error>`        |
| `load_from_path`  | 从字符串路径加载         | `Result<(), Error>`        |
| `load_or_replace` | 强制加载（替换已存在的） | `Result<(), Error>`        |
| `get`             | 获取库信息               | `Option<Arc<LibraryInfo>>` |
| `contains`        | 检查库是否存在           | `bool`                     |
| `list`            | 列出所有库名称           | `Vec<String>`              |
| `count`           | 获取库数量               | `usize`                    |
| `unload`          | 卸载库                   | `Result<PathBuf, Error>`   |
| `unload_force`    | 强制卸载库               | `Result<PathBuf, Error>`   |
| `reload`          | 重新加载库               | `Result<(), Error>`        |
| `clear`           | 清空所有库               | `Result<(), Error>`        |
| `clear_force`     | 强制清空所有库           | `()`                       |

### Error

错误类型枚举。

| 变体          | 说明         | 参数             |
| ------------- | ------------ | ---------------- |
| `NotFound`    | 库不存在     | 库名称（String） |
| `Exists`      | 库已存在     | 库名称（String） |
| `InUse`       | 库正在使用中 | 库名称（String） |
| `LoadFailed`  | 库加载失败   | 路径和底层错误   |
| `InvalidPath` | 无效的库路径 | 路径（PathBuf）  |

## 使用场景

### 插件系统

```rust
use puniyu_library::{LibraryRegistry, Error};
use std::path::Path;

struct PluginManager;

impl PluginManager {
    fn load_plugin(&self, path: &Path) -> Result<(), Error> {
        // 加载插件库
        LibraryRegistry::load(path)?;

        // 获取并初始化插件
        if let Some(lib) = LibraryRegistry::get(
            path.file_name().unwrap().to_str().unwrap()
        ) {
            unsafe {
                let init: libloading::Symbol<fn()> =
                    lib.library().get(b"plugin_init")?;
                init();
            }
        }

        Ok(())
    }

    fn unload_plugin(&self, name: &str) -> Result<(), Error> {
        // 调用清理函数
        if let Some(lib) = LibraryRegistry::get(name) {
            unsafe {
                let cleanup: libloading::Symbol<fn()> =
                    lib.library().get(b"plugin_cleanup")?;
                cleanup();
            }
        }

        // 卸载库
        LibraryRegistry::unload(name)?;
        Ok(())
    }

    fn reload_plugin(&self, name: &str) -> Result<(), Error> {
        self.unload_plugin(name)?;

        // 重新加载
        if let Some(lib) = LibraryRegistry::get(name) {
            let path = lib.path().to_path_buf();
            LibraryRegistry::load(&path)?;
        }

        Ok(())
    }
}
```

### 热重载开发

```rust
use puniyu_library::LibraryRegistry;
use std::path::Path;
use std::thread;
use std::time::Duration;

fn watch_and_reload(plugin_path: &Path) {
    // 初始加载
    LibraryRegistry::load(plugin_path).ok();

    let name = plugin_path.file_name().unwrap().to_str().unwrap();

    loop {
        thread::sleep(Duration::from_secs(5));

        // 检测文件变化后重新加载
        if file_changed(plugin_path) {
            println!("检测到变化，重新加载...");
            if let Err(e) = LibraryRegistry::reload(name) {
                eprintln!("重载失败: {}", e);
            } else {
                println!("重载成功");
            }
        }
    }
}

fn file_changed(path: &Path) -> bool {
    // 实现文件变化检测逻辑
    false
}
```

### 动态模块加载

```rust
use puniyu_library::{LibraryRegistry, LibraryInfo};
use std::path::PathBuf;

struct ModuleLoader {
    modules_dir: PathBuf,
}

impl ModuleLoader {
    fn load_all_modules(&self) -> Result<(), Box<dyn std::error::Error>> {
        let entries = std::fs::read_dir(&self.modules_dir)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            // 只加载动态库文件
            if path.extension().and_then(|s| s.to_str()) == Some("dll")
                || path.extension().and_then(|s| s.to_str()) == Some("so") {
                match LibraryRegistry::load(&path) {
                    Ok(_) => println!("已加载模块: {:?}", path),
                    Err(e) => eprintln!("加载失败: {} - {}", path.display(), e),
                }
            }
        }

        Ok(())
    }

    fn list_loaded_modules(&self) -> Vec<String> {
        LibraryRegistry::list()
    }

    fn unload_all_modules(&self) -> Result<(), Box<dyn std::error::Error>> {
        LibraryRegistry::clear()?;
        Ok(())
    }
}
```

## 注意事项

### 线程安全

所有操作都是线程安全的，可以在多线程环境中安全使用：

```rust
use puniyu_library::LibraryRegistry;
use std::thread;

let handles: Vec<_> = (0..10).map(|i| {
    thread::spawn(move || {
        if let Some(lib) = LibraryRegistry::get("plugin.dll") {
            // 安全地使用库
        }
    })
}).collect();

for handle in handles {
    handle.join().unwrap();
}
```

### 引用计数

库使用 `Arc` 进行引用计数，只有当没有其他引用时才能卸载：

```rust
use puniyu_library::LibraryRegistry;

LibraryRegistry::load_from_path("plugin.dll")?;

// 获取引用
let lib1 = LibraryRegistry::get("plugin.dll");
let lib2 = LibraryRegistry::get("plugin.dll");

// 此时无法卸载，因为有外部引用
assert!(LibraryRegistry::unload("plugin.dll").is_err());

// 释放所有引用后才能卸载
drop(lib1);
drop(lib2);
assert!(LibraryRegistry::unload("plugin.dll").is_ok());
```

### 平台差异

不同平台的动态库文件扩展名不同：

- Windows: `.dll`
- Linux: `.so`
- macOS: `.dylib`

建议使用条件编译处理：

```rust
#[cfg(target_os = "windows")]
const LIB_EXT: &str = "dll";

#[cfg(target_os = "linux")]
const LIB_EXT: &str = "so";

#[cfg(target_os = "macos")]
const LIB_EXT: &str = "dylib";
```

### 安全性

动态库加载本质上是不安全的操作，使用时需要注意：

1. 确保库文件来源可信
2. 验证库的签名和完整性
3. 使用 `unsafe` 块调用库函数时要格外小心
4. 避免在库卸载后继续使用其中的函数指针

## API 改进

相比旧版本，新 API 的改进：

| 旧 API           | 新 API             | 改进               |
| ---------------- | ------------------ | ------------------ |
| `load_library`   | `load`             | 更简洁的命名       |
| -                | `load_from_path`   | 新增便捷方法       |
| -                | `load_or_replace`  | 新增强制加载功能   |
| `get_library`    | `get`              | 更简洁的命名       |
| -                | `contains`         | 新增存在性检查     |
| -                | `list`             | 新增列出所有库功能 |
| -                | `count`            | 新增计数功能       |
| `remove_library` | `unload`           | 更准确的命名       |
| -                | `unload_force`     | 新增强制卸载功能   |
| `reload_library` | `reload`           | 更简洁的命名       |
| -                | `clear`            | 新增清空功能       |
| -                | `clear_force`      | 新增强制清空功能   |
| `From<PathBuf>`  | `LibraryInfo::new` | 更安全的错误处理   |

## 文档

完整的 API 文档请访问 [docs.rs](https://docs.rs/puniyu_library)。

## 许可证

本项目采用 [LGPL-3.0](../../LICENSE) 许可证。

## 相关链接

- [GitHub 仓库](https://github.com/puniyu/puniyu)
- [Puniyu 框架](https://github.com/puniyu/puniyu)
- [libloading](https://docs.rs/libloading) - 底层动态库加载库
