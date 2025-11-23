# Changelog

## [0.5.2](https://github.com/puniyu/puniyu/compare/core-v0.5.1...core-v0.5.2) (2025-11-23)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_bus bumped from 0.4.3 to 0.4.4

## [0.5.1](https://github.com/puniyu/puniyu/compare/core-v0.5.0...core-v0.5.1) (2025-11-23)


### 🔧 其他更新

* **core:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_server bumped from 0.5.0 to 0.5.1
    * puniyu_config bumped from 0.5.0 to 0.5.1
    * puniyu_bus bumped from 0.4.2 to 0.4.3
    * puniyu_common bumped from 0.5.0 to 0.5.1
    * puniyu_types bumped from 0.5.0 to 0.5.1
    * puniyu_registry bumped from 0.5.0 to 0.5.1

## [0.5.0](https://github.com/puniyu/puniyu/compare/core-v0.4.1...core-v0.5.0) (2025-11-23)


### ✨ 新功能

* **adapter:** 支持配置文件读取功能 ([a9fc6e2](https://github.com/puniyu/puniyu/commit/a9fc6e2aed53370db0c78a0035c37eec53114445))
* **config:** 重构配置管理模块以支持动态注册与热重载 ([f3234c1](https://github.com/puniyu/puniyu/commit/f3234c16ea7d49b4cae2cdd0bda024f390778497))


### 🔧 其他更新

* **deps:** 更新依赖版本 ([63551d7](https://github.com/puniyu/puniyu/commit/63551d722a048e4e5e0479fe421d7190a8b7c7e9))


### ♻️ 代码重构

* **adapter:** 优化 Avatar 类型实现并添加服务器 logo 接口 ([952c18b](https://github.com/puniyu/puniyu/commit/952c18b3008a5e31fd00127dc6d2fb55568c9796))
* **config:** 添加适配器配置系统支持 ([5358888](https://github.com/puniyu/puniyu/commit/5358888a950988a6be04c9b539bdac95c9b09b8d))
* **core:** 调整Bot模块导出方式 ([d94d88c](https://github.com/puniyu/puniyu/commit/d94d88cf891d9e222fb996c54895fa062bc17fe7))
* 重构工作区crates ([#53](https://github.com/puniyu/puniyu/issues/53)) ([f55ab51](https://github.com/puniyu/puniyu/commit/f55ab519b9275c19773f16acfddff0c44db8e48c))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_server bumped from 0.4.1 to 0.5.0
    * puniyu_config bumped from 0.4.1 to 0.5.0
    * puniyu_bus bumped from 0.4.1 to 0.4.2
    * puniyu_common bumped from 0.4.1 to 0.5.0
    * puniyu_types bumped from 0.4.1 to 0.5.0
    * puniyu_registry bumped from 0.4.1 to 0.5.0

## [0.4.1](https://github.com/puniyu/puniyu/compare/core-v0.4.0...core-v0.4.1) (2025-11-16)


### 🐛 错误修复

* **path:** 重构工作目录设置逻辑 ([df51eac](https://github.com/puniyu/puniyu/commit/df51eac9e4fe92b11df2867ebdeca78fe62b2022))


### 🔧 其他更新

* **core:** 导出 puniyu_common 的 path 模块 ([a95c43e](https://github.com/puniyu/puniyu/commit/a95c43e1104b4bb4aadaeaddc3d07f43dc083968))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_config bumped from 0.4.0 to 0.4.1
    * puniyu_bot bumped from 0.4.0 to 0.4.1
    * puniyu_task bumped from 0.4.0 to 0.4.1
    * puniyu_server bumped from 0.4.0 to 0.4.1
    * puniyu_builder bumped from 0.4.0 to 0.4.1
    * puniyu_registry bumped from 0.4.0 to 0.4.1
    * puniyu_common bumped from 0.4.0 to 0.4.1
    * puniyu_event_bus bumped from 0.4.0 to 0.4.1

## [0.4.0](https://github.com/puniyu/puniyu/compare/core-v0.3.1...core-v0.4.0) (2025-11-16)


### 🔧 其他更新

* **core:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_config bumped from 0.3.0 to 0.4.0
    * puniyu_bot bumped from 0.3.0 to 0.4.0
    * puniyu_task bumped from 0.1.10 to 0.4.0
    * puniyu_server bumped from 0.2.0 to 0.4.0
    * puniyu_builder bumped from 0.3.0 to 0.4.0
    * puniyu_registry bumped from 0.3.0 to 0.4.0
    * puniyu_common bumped from 0.3.0 to 0.4.0
    * puniyu_event_bus bumped from 0.3.0 to 0.4.0

## [0.3.1](https://github.com/puniyu/puniyu/compare/core-v0.3.0...core-v0.3.1) (2025-11-15)


### 🔧 其他更新

* **release:** 调整发布配置并更新依赖版本 ([381ca55](https://github.com/puniyu/puniyu/commit/381ca558c7ca6b4bcc9b9386ecc228e8679f7305))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_config bumped from 0.2.0 to 0.3.0
    * puniyu_bot bumped from 0.2.0 to 0.3.0
    * puniyu_task bumped from 0.1.9 to 0.1.10
    * puniyu_builder bumped from 0.2.0 to 0.3.0
    * puniyu_registry bumped from 0.2.0 to 0.3.0
    * puniyu_common bumped from 0.2.0 to 0.3.0
    * puniyu_event_bus bumped from 0.2.0 to 0.3.0

## [0.3.0](https://github.com/puniyu/puniyu/compare/core-v0.2.0...core-v0.3.0) (2025-11-15)


### ✨ 新功能

* **core:** 将 puniyu_logger 设置为可选依赖并更新版本获取逻辑 ([f49677a](https://github.com/puniyu/puniyu/commit/f49677ad1bcf2c5b64ff699241c427a7a7300cc0))


### ♻️ 代码重构

* **config:** 重构配置文件监听器实现 ([4c99137](https://github.com/puniyu/puniyu/commit/4c9913784f5a40bcb8d13494121489ea86ce17c4))
* **project:** 重构项目结构 ([b1b389f](https://github.com/puniyu/puniyu/commit/b1b389f25dae5899b49133be88ba348930117972))
