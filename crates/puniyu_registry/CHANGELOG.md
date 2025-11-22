# Changelog

## [0.4.2](https://github.com/puniyu/puniyu/compare/registry-v0.4.1...registry-v0.4.2) (2025-11-22)


### 🐛 错误修复

* **puniyu_registry:** 为 adapter 特性添加 server 支持 ([1c71c3e](https://github.com/puniyu/puniyu/commit/1c71c3e3f223048a55d5bf8bd27f6654e0015064))


### ♻️ 代码重构

* **error:** 统一错误处理机制 ([586272d](https://github.com/puniyu/puniyu/commit/586272ddf8828cfa7af809bc01de95d1245c4d9b))
* 重构工作区crates ([#53](https://github.com/puniyu/puniyu/issues/53)) ([f55ab51](https://github.com/puniyu/puniyu/commit/f55ab519b9275c19773f16acfddff0c44db8e48c))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.4.1 to 0.4.2
    * puniyu_types bumped from 0.4.1 to 0.4.2
    * puniyu_library bumped from 0.4.1 to 0.4.2
    * puniyu_config bumped from 0.4.1 to 0.4.2

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
