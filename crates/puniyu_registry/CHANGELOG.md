# Changelog

## [0.5.10](https://github.com/puniyu/puniyu/compare/registry-v0.5.9...registry-v0.5.10) (2025-11-23)


### ♻️ 代码重构

* **command:** 重构命令处理结果类型和参数验证 ([58d4eeb](https://github.com/puniyu/puniyu/commit/58d4eebb41cacabc7663b40a93181b789feb1e0a))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.9 to 0.5.10
    * puniyu_types bumped from 0.5.9 to 0.5.10
    * puniyu_config bumped from 0.5.9 to 0.5.10

## [0.5.9](https://github.com/puniyu/puniyu/compare/registry-v0.5.8...registry-v0.5.9) (2025-11-23)


### 🔧 其他更新

* **registry:** 在加载日志中显示插件和适配器版本号 ([0184a74](https://github.com/puniyu/puniyu/commit/0184a74b1dc267690d15bd8e1f6ba25669f7c194))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.8 to 0.5.9
    * puniyu_types bumped from 0.5.8 to 0.5.9
    * puniyu_config bumped from 0.5.8 to 0.5.9

## [0.5.8](https://github.com/puniyu/puniyu/compare/registry-v0.5.7...registry-v0.5.8) (2025-11-23)


### 🔧 其他更新

* **registry:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.7 to 0.5.8
    * puniyu_types bumped from 0.5.7 to 0.5.8
    * puniyu_config bumped from 0.5.7 to 0.5.8

## [0.5.7](https://github.com/puniyu/puniyu/compare/registry-v0.5.6...registry-v0.5.7) (2025-11-23)


### 🔧 其他更新

* **registry:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.6 to 0.5.7
    * puniyu_types bumped from 0.5.6 to 0.5.7
    * puniyu_config bumped from 0.5.6 to 0.5.7

## [0.5.6](https://github.com/puniyu/puniyu/compare/registry-v0.5.5...registry-v0.5.6) (2025-11-23)


### 🔧 其他更新

* **deps:** 更新依赖配置 ([84a5c1e](https://github.com/puniyu/puniyu/commit/84a5c1ea0f1d31510a3ee438d4fe408f28686022))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.4 to 0.5.6
    * puniyu_types bumped from 0.5.4 to 0.5.6
    * puniyu_config bumped from 0.5.4 to 0.5.6

## [0.5.5](https://github.com/puniyu/puniyu/compare/registry-v0.5.4...registry-v0.5.5) (2025-11-23)


### 🔧 其他更新

* **puniyu_registry:** 修补依赖 ([e99a202](https://github.com/puniyu/puniyu/commit/e99a202288428974ea55293c189a70bf3f194110))

## [0.5.4](https://github.com/puniyu/puniyu/compare/registry-v0.5.3...registry-v0.5.4) (2025-11-23)


### 🐛 错误修复

* **puniyu_registry:** 补充缺少的依赖 ([8a33e67](https://github.com/puniyu/puniyu/commit/8a33e6775884cf8e65cf3f3baab536d0e3fe4dd6))

## [0.5.3](https://github.com/puniyu/puniyu/compare/registry-v0.5.2...registry-v0.5.3) (2025-11-23)


### 🔧 其他更新

* **core:** 支持设置app_logo ([3f4f713](https://github.com/puniyu/puniyu/commit/3f4f71344917f671468edfef639ec201440a1251))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.1 to 0.5.4
    * puniyu_types bumped from 0.5.1 to 0.5.4
    * puniyu_config bumped from 0.5.1 to 0.5.4

## [0.5.2](https://github.com/puniyu/puniyu/compare/registry-v0.5.1...registry-v0.5.2) (2025-11-23)


### 🔧 其他更新

* **deps:** 添加 futures 依赖项 ([45fc941](https://github.com/puniyu/puniyu/commit/45fc941dced932cbd5e8642f45af1d115d80fea1))

## [0.5.1](https://github.com/puniyu/puniyu/compare/registry-v0.5.0...registry-v0.5.1) (2025-11-23)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.0 to 0.5.1
    * puniyu_types bumped from 0.5.0 to 0.5.1
    * puniyu_config bumped from 0.5.0 to 0.5.1

## [0.5.0](https://github.com/puniyu/puniyu/compare/registry-v0.4.1...registry-v0.5.0) (2025-11-23)


### ✨ 新功能

* **adapter:** 支持配置文件读取功能 ([a9fc6e2](https://github.com/puniyu/puniyu/commit/a9fc6e2aed53370db0c78a0035c37eec53114445))
* **config:** 将配置序列化格式从 JSON 切换为 TOML ([48fc976](https://github.com/puniyu/puniyu/commit/48fc976274386311b62fa1d344807a68432b99f0))
* **config:** 重构配置管理模块以支持动态注册与热重载 ([f3234c1](https://github.com/puniyu/puniyu/commit/f3234c16ea7d49b4cae2cdd0bda024f390778497))
* **plugin:** 添加插件配置支持 ([dc7d1eb](https://github.com/puniyu/puniyu/commit/dc7d1ebcf2245f53f3a58b203edd405aa7cc8c1c))


### 🐛 错误修复

* **puniyu_registry:** 为 adapter 特性添加 server 支持 ([1c71c3e](https://github.com/puniyu/puniyu/commit/1c71c3e3f223048a55d5bf8bd27f6654e0015064))


### ♻️ 代码重构

* **config:** 添加适配器配置系统支持 ([5358888](https://github.com/puniyu/puniyu/commit/5358888a950988a6be04c9b539bdac95c9b09b8d))
* **error:** 统一错误处理机制 ([586272d](https://github.com/puniyu/puniyu/commit/586272ddf8828cfa7af809bc01de95d1245c4d9b))
* 重构工作区crates ([#53](https://github.com/puniyu/puniyu/issues/53)) ([f55ab51](https://github.com/puniyu/puniyu/commit/f55ab519b9275c19773f16acfddff0c44db8e48c))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.4.1 to 0.5.0
    * puniyu_types bumped from 0.4.1 to 0.5.0
    * puniyu_library bumped from 0.4.1 to 0.5.0
    * puniyu_config bumped from 0.4.1 to 0.5.0

## [0.4.1](https://github.com/puniyu/puniyu/compare/registry-v0.4.0...registry-v0.4.1) (2025-11-16)


### 🔧 其他更新

* **registry:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_task bumped from 0.4.0 to 0.4.1
    * puniyu_bot bumped from 0.4.0 to 0.4.1
    * puniyu_command bumped from 0.4.0 to 0.4.1
    * puniyu_common bumped from 0.4.0 to 0.4.1
    * puniyu_builder bumped from 0.4.0 to 0.4.1
    * puniyu_library bumped from 0.4.0 to 0.4.1
    * puniyu_config bumped from 0.4.0 to 0.4.1

## [0.4.0](https://github.com/puniyu/puniyu/compare/registry-v0.3.0...registry-v0.4.0) (2025-11-16)


### ✨ 新功能

* **adapter:** 初始化适配器数据目录结构 ([0e7413b](https://github.com/puniyu/puniyu/commit/0e7413b8790cb2b6c7f1bf7ed43046be1169cfaf))
* **adapter:** 添加适配器卸载功能 ([86ca219](https://github.com/puniyu/puniyu/commit/86ca2194f319f11476c729283cebf373da7203c0))
* **bot:** 重构Bot注册表与消息发送功能 ([bb4e391](https://github.com/puniyu/puniyu/commit/bb4e3912885ab95e3cd200048240d232f8257279))
* **core:** 自动创建插件和适配器数据目录 ([881a42e](https://github.com/puniyu/puniyu/commit/881a42ece6fb13ae8ad11c94e01e9c4463a32ec4))
* **registry:** 引入配置驱动的插件/适配器强制加载机制 ([a7fd2f2](https://github.com/puniyu/puniyu/commit/a7fd2f2dc0bb0427eaf2731d0de752d9ad2d2d39))


### 🔧 其他更新

* **config:** 初始化配置模块并添加 README 文档 ([6109f15](https://github.com/puniyu/puniyu/commit/6109f151b73d1ad24c5237f5602aad40a7fbbba4))
* **config:** 初始化配置模块并添加 README 文档 ([e9099fc](https://github.com/puniyu/puniyu/commit/e9099fc66da3b14413f3851c82771f148cff5c10))
* release main ([b22b2f0](https://github.com/puniyu/puniyu/commit/b22b2f017c88290346428c229c975cc570bc70d1))
* release main ([be9bdfe](https://github.com/puniyu/puniyu/commit/be9bdfe1fcee37185d800f1cf0bd5758d83776e1))
* **release:** 调整发布配置并更新依赖版本 ([381ca55](https://github.com/puniyu/puniyu/commit/381ca558c7ca6b4bcc9b9386ecc228e8679f7305))


### ♻️ 代码重构

* **adapter:** 简化适配器加载逻辑并移除ABI版本检查 ([30104ed](https://github.com/puniyu/puniyu/commit/30104edcd5c1e81ffb87a4da6718bbc0399ff941))
* **adapter:** 重构适配器模块结构和依赖关系 ([1cedfac](https://github.com/puniyu/puniyu/commit/1cedfac70a93d071b25ea2721df7c9f41123e1bf))
* **library:** 重构库存储接口 ([5918516](https://github.com/puniyu/puniyu/commit/5918516a65943caae4a7bacf29a58dcb96e3ca52))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_task bumped from 0.1.10 to 0.4.0
    * puniyu_bot bumped from 0.3.0 to 0.4.0
    * puniyu_command bumped from 0.3.0 to 0.4.0
    * puniyu_common bumped from 0.3.0 to 0.4.0
    * puniyu_builder bumped from 0.3.0 to 0.4.0
    * puniyu_library bumped from 0.3.0 to 0.4.0
    * puniyu_config bumped from 0.3.0 to 0.4.0

## [0.3.0](https://github.com/puniyu/puniyu/compare/v0.2.0...v0.3.0) (2025-11-15)


### ✨ 新功能

* **adapter:** 初始化适配器数据目录结构 ([0e7413b](https://github.com/puniyu/puniyu/commit/0e7413b8790cb2b6c7f1bf7ed43046be1169cfaf))
* **adapter:** 添加适配器卸载功能 ([86ca219](https://github.com/puniyu/puniyu/commit/86ca2194f319f11476c729283cebf373da7203c0))
* **bot:** 重构Bot注册表与消息发送功能 ([bb4e391](https://github.com/puniyu/puniyu/commit/bb4e3912885ab95e3cd200048240d232f8257279))
* **core:** 自动创建插件和适配器数据目录 ([881a42e](https://github.com/puniyu/puniyu/commit/881a42ece6fb13ae8ad11c94e01e9c4463a32ec4))
* **registry:** 引入配置驱动的插件/适配器强制加载机制 ([a7fd2f2](https://github.com/puniyu/puniyu/commit/a7fd2f2dc0bb0427eaf2731d0de752d9ad2d2d39))


### 🔧 其他更新

* **config:** 初始化配置模块并添加 README 文档 ([6109f15](https://github.com/puniyu/puniyu/commit/6109f151b73d1ad24c5237f5602aad40a7fbbba4))
* **config:** 初始化配置模块并添加 README 文档 ([e9099fc](https://github.com/puniyu/puniyu/commit/e9099fc66da3b14413f3851c82771f148cff5c10))
* release main ([be9bdfe](https://github.com/puniyu/puniyu/commit/be9bdfe1fcee37185d800f1cf0bd5758d83776e1))
* **release:** 调整发布配置并更新依赖版本 ([381ca55](https://github.com/puniyu/puniyu/commit/381ca558c7ca6b4bcc9b9386ecc228e8679f7305))


### ♻️ 代码重构

* **adapter:** 简化适配器加载逻辑并移除ABI版本检查 ([30104ed](https://github.com/puniyu/puniyu/commit/30104edcd5c1e81ffb87a4da6718bbc0399ff941))
* **adapter:** 重构适配器模块结构和依赖关系 ([1cedfac](https://github.com/puniyu/puniyu/commit/1cedfac70a93d071b25ea2721df7c9f41123e1bf))
* **library:** 重构库存储接口 ([5918516](https://github.com/puniyu/puniyu/commit/5918516a65943caae4a7bacf29a58dcb96e3ca52))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_task bumped from 0.1.9 to 0.1.10
    * puniyu_bot bumped from 0.2.0 to 0.3.0
    * puniyu_command bumped from 0.2.0 to 0.3.0
    * puniyu_common bumped from 0.2.0 to 0.3.0
    * puniyu_builder bumped from 0.2.0 to 0.3.0
    * puniyu_library bumped from 0.2.0 to 0.3.0
    * puniyu_config bumped from 0.2.0 to 0.3.0

## [0.2.0](https://github.com/puniyu/puniyu/compare/registry-v0.1.8...registry-v0.2.0) (2025-11-15)


### ✨ 新功能

* **adapter:** 初始化适配器数据目录结构 ([0e7413b](https://github.com/puniyu/puniyu/commit/0e7413b8790cb2b6c7f1bf7ed43046be1169cfaf))
* **adapter:** 添加适配器卸载功能 ([86ca219](https://github.com/puniyu/puniyu/commit/86ca2194f319f11476c729283cebf373da7203c0))
* **bot:** 重构Bot注册表与消息发送功能 ([bb4e391](https://github.com/puniyu/puniyu/commit/bb4e3912885ab95e3cd200048240d232f8257279))
* **core:** 自动创建插件和适配器数据目录 ([881a42e](https://github.com/puniyu/puniyu/commit/881a42ece6fb13ae8ad11c94e01e9c4463a32ec4))
* **registry:** 引入配置驱动的插件/适配器强制加载机制 ([a7fd2f2](https://github.com/puniyu/puniyu/commit/a7fd2f2dc0bb0427eaf2731d0de752d9ad2d2d39))


### 🔧 其他更新

* **config:** 初始化配置模块并添加 README 文档 ([6109f15](https://github.com/puniyu/puniyu/commit/6109f151b73d1ad24c5237f5602aad40a7fbbba4))
* **config:** 初始化配置模块并添加 README 文档 ([e9099fc](https://github.com/puniyu/puniyu/commit/e9099fc66da3b14413f3851c82771f148cff5c10))


### ♻️ 代码重构

* **adapter:** 简化适配器加载逻辑并移除ABI版本检查 ([30104ed](https://github.com/puniyu/puniyu/commit/30104edcd5c1e81ffb87a4da6718bbc0399ff941))
* **adapter:** 重构适配器模块结构和依赖关系 ([1cedfac](https://github.com/puniyu/puniyu/commit/1cedfac70a93d071b25ea2721df7c9f41123e1bf))
* **library:** 重构库存储接口 ([5918516](https://github.com/puniyu/puniyu/commit/5918516a65943caae4a7bacf29a58dcb96e3ca52))
