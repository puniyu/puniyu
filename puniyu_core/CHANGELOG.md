# Changelog

## [0.3.0](https://github.com/puniyu/puniyu/compare/core-v0.2.0...core-v0.3.0) (2025-10-25)


### ✨ 新功能

* **adapter:** 导出适配器相关类型和构建器 ([73e9e76](https://github.com/puniyu/puniyu/commit/73e9e764603f1c2fb493a86da9c5d815fb95e55e))
* **bot:** 重构Bot注册表与消息发送功能 ([bb4e391](https://github.com/puniyu/puniyu/commit/bb4e3912885ab95e3cd200048240d232f8257279))
* **config:** 将配置模块拆分为独立的puniyu_config crate ([10351d6](https://github.com/puniyu/puniyu/commit/10351d6451cd53bcc35dadbeb885553cfb3d66d3))
* **core:** 实现事件驱动和账户信息功能 ([8e05ebd](https://github.com/puniyu/puniyu/commit/8e05ebde81c661f94b4c7599e72971de06c54173))
* **core:** 支持自定义程序名称 ([b7f11f7](https://github.com/puniyu/puniyu/commit/b7f11f76fc32deb2e947d78c7ecdaa47712261e1))
* **core:** 添加应用启动欢迎界面和插件并行加载 ([6b438da](https://github.com/puniyu/puniyu/commit/6b438dacb03abc83a36a2501c23a01260d06e985))
* **core:** 添加版本信息构建脚本和版本结构体 ([8c3efd2](https://github.com/puniyu/puniyu/commit/8c3efd2897b776447b65fe7147d36dce3b9d2e6e))
* **core:** 自动创建插件和适配器数据目录 ([881a42e](https://github.com/puniyu/puniyu/commit/881a42ece6fb13ae8ad11c94e01e9c4463a32ec4))
* **core:** 重构事件处理和插件系统 ([a62a0ed](https://github.com/puniyu/puniyu/commit/a62a0ed3c658c870c49e7354d992ab13cddd928a))
* **core:** 重构配置模块并添加 Bot 注册功能 ([52c8acb](https://github.com/puniyu/puniyu/commit/52c8acb748c79018429d3f6a787238fd1b2bf14a))
* **core:** 重构配置模块并添加热重载功能 ([eee5739](https://github.com/puniyu/puniyu/commit/eee57396599d5c10ab6cd3520dfc07b5e1fa878d))
* **plugin:** 重构任务管理器以支持动态任务注册与移除 ([7ecb688](https://github.com/puniyu/puniyu/commit/7ecb6883b245b699c2bcc4bbaaad5b4af372959b))
* **plugin:** 重构插件体系并添加任务调度功能 ([5217c68](https://github.com/puniyu/puniyu/commit/5217c68a2ed6c29a889c3199c9992de6c7142e60))
* **puniyu_macro:** 添加 proc-macro 用于创建插件 ([d933976](https://github.com/puniyu/puniyu/commit/d9339769ae5e91d57a7f0ca0e0d21c1a55c83ad1))
* **segment:** 添加消息元素 ([4da83d6](https://github.com/puniyu/puniyu/commit/4da83d6d7d98d44a9c295054dbea228b48a464e0))
* **server:** 添加 HTTP 服务器模块并初始化配置 ([7e83259](https://github.com/puniyu/puniyu/commit/7e832595521a21081530a8c9c318d416e747ee0b))
* **task:** 添加定时任务模块 ([e9d823b](https://github.com/puniyu/puniyu/commit/e9d823b5d6ccd089a26166c59e2b67ff8dbd75c1))
* **utils:** 新增配置文件处理功能 ([c73a38e](https://github.com/puniyu/puniyu/commit/c73a38efe62140edc2170a33ceae083f069c51de))


### 🐛 错误修复

* **workflows:** 修复下载构建产物时的正则表达式语法 ([da3a2bb](https://github.com/puniyu/puniyu/commit/da3a2bbc2c606566bb30c14206665b6e6434e18f))


### ⚡️ 性能优化

* **core:** 优化 EventEmitter 结构中的锁机制 ([18b77e1](https://github.com/puniyu/puniyu/commit/18b77e1685004da6a758afb9ee8bbf60ba5946a4))


### 🔧 其他更新

* **config:** 初始化配置模块并添加 README 文档 ([6109f15](https://github.com/puniyu/puniyu/commit/6109f151b73d1ad24c5237f5602aad40a7fbbba4))
* 增加适配器基类实现 ([b81451e](https://github.com/puniyu/puniyu/commit/b81451e6a6f7847a93ed3465bf1beb8e3552dcbf))
* 移除冗余代码 ([54e0e89](https://github.com/puniyu/puniyu/commit/54e0e89e2163b3b8d9c3808bc8580b07c692fb99))


### ♻️ 代码重构

* **adapter:** 简化适配器加载逻辑并移除ABI版本检查 ([30104ed](https://github.com/puniyu/puniyu/commit/30104edcd5c1e81ffb87a4da6718bbc0399ff941))
* **adapter:** 重构适配器模块结构和依赖关系 ([1cedfac](https://github.com/puniyu/puniyu/commit/1cedfac70a93d071b25ea2721df7c9f41123e1bf))
* **core:** 重构消息模块 ([5634d99](https://github.com/puniyu/puniyu/commit/5634d99aa2b55841c86f135ba925cf19bc237efd))
* **core:** 重构配置模块并优化系统信息获取 ([4d56f79](https://github.com/puniyu/puniyu/commit/4d56f7941e213391189af0da8c05c1383285434d))
* **message:** 重构消息模块 ([950a65e](https://github.com/puniyu/puniyu/commit/950a65eb774fa1fe8a104a6375193b5e2a1e2f59))
* **message:** 重构音乐元素创建逻辑 ([203e2e7](https://github.com/puniyu/puniyu/commit/203e2e7f84ca78840d8eba41cd931a27990da544))
* **plugin:** 优化插件和适配器加载逻辑 ([a0a529a](https://github.com/puniyu/puniyu/commit/a0a529a6beb6b922d1bcb83dd5098807d9bf078d))
* **puniyu_utils:** 重构配置模块并优化路径处理 ([decb3dc](https://github.com/puniyu/puniyu/commit/decb3dcad12253aa2f4aa5959524240784d654b8))
* 重构项目并更改名称 ([f2d05b1](https://github.com/puniyu/puniyu/commit/f2d05b110676d49ff33237edd0a61f1a77a98652))
* 重构项目配置 ([883dfa4](https://github.com/puniyu/puniyu/commit/883dfa4fb525d7e5c27821026d727e3d8eda8600))
