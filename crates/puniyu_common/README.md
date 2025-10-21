# puniyu_common

puniyu公共函数库

## 概述

`puniyu_common` 是 puniyu 项目中的基础公共库，提供了项目中通用的工具函数、常量定义和系统级功能。该库旨在为整个 puniyu
生态系统提供一致的基础支持。

## 核心模块

### app 模块

提供应用程序级别的全局变量：

- `APP_NAME`: 使用 `OnceLock` 实现的全局应用程序名称静态变量

### path 模块

定义项目中常用的路径常量，所有路径都使用 `LazyLock<PathBuf>` 延迟初始化：

- `BASE_DIR`: 应用根目录
- `LOG_DIR`: 日志文件夹路径
- `APP_DIR`: 应用数据文件夹路径
- `CONFIG_DIR`: 配置文件夹路径
- `TEMP_DIR`: 临时文件夹路径
- `PLUGIN_DIR`: 插件文件夹路径
- `ADAPTER_DIR`: 适配器文件夹路径

### system 模块

系统信息相关功能，封装了系统资源监控：

- `SystemInfo`: 系统信息结构体，包含 CPU、内存、磁盘、主机和 GPU 信息
- `BotStatusInfo`: Bot 状态信息，包括进程 PID、CPU 使用率、内存使用情况等
- 封装了 `puniyu_system_info` crate 提供的底层系统信息获取功能

### toml 模块

TOML 配置文件操作工具集：

- `read_config`: 读取 TOML 配置文件
- `write_config`: 写入 TOML 配置文件
- `update_config`: 更新 TOML 配置文件
- `merge_config`: 合并两个 TOML 配置结构体
- `delete_config`: 删除 TOML 配置文件中的指定节点

### error 模块

定义通用错误类型：

- `Config`: 配置文件相关错误枚举，包括读取、写入、解析、权限等错误类型

## 许可证

本项目采用 [LGPL](../../LICENSE) 许可证。