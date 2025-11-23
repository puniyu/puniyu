# Changelog

## [0.5.0](https://github.com/puniyu/puniyu/compare/adapter-v0.4.1...adapter-v0.5.0) (2025-11-23)


### ✨ 新功能

* **adapter:** 支持配置文件读取功能 ([a9fc6e2](https://github.com/puniyu/puniyu/commit/a9fc6e2aed53370db0c78a0035c37eec53114445))
* **config:** 将配置序列化格式从 JSON 切换为 TOML ([48fc976](https://github.com/puniyu/puniyu/commit/48fc976274386311b62fa1d344807a68432b99f0))
* **config:** 重构配置管理模块以支持动态注册与热重载 ([f3234c1](https://github.com/puniyu/puniyu/commit/f3234c16ea7d49b4cae2cdd0bda024f390778497))


### 🐛 错误修复

* **console:** 优化控制台适配器配置与资源管理 ([31184f1](https://github.com/puniyu/puniyu/commit/31184f134328fc0b193972675e2274ea53a38864))


### ♻️ 代码重构

* **adapter:** 独立account模块 ([0f4c175](https://github.com/puniyu/puniyu/commit/0f4c1758ecc817c2048fec38265edbf630252cb0))
* **config:** 添加适配器配置系统支持 ([5358888](https://github.com/puniyu/puniyu/commit/5358888a950988a6be04c9b539bdac95c9b09b8d))
* **contact:** 重构联系人类型系统，引入 trait 抽象 ([c403f08](https://github.com/puniyu/puniyu/commit/c403f08b8aaebfa071995c8179c9d75a8ae532d9))
* **error:** 统一错误处理机制 ([586272d](https://github.com/puniyu/puniyu/commit/586272ddf8828cfa7af809bc01de95d1245c4d9b))
* **workspace:** 重构项目结构和依赖管理 ([520087e](https://github.com/puniyu/puniyu/commit/520087e38b49ff9e3aabe04be946038b81114b4d))
* 重构工作区crates ([#53](https://github.com/puniyu/puniyu/issues/53)) ([f55ab51](https://github.com/puniyu/puniyu/commit/f55ab519b9275c19773f16acfddff0c44db8e48c))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_config bumped from 0.4.1 to 0.5.0
    * puniyu_macros bumped from 0.4.1 to 0.5.0
    * puniyu_common bumped from 0.4.1 to 0.5.0
    * puniyu_types bumped from 0.4.1 to 0.5.0
    * puniyu_registry bumped from 0.4.1 to 0.5.0
    * puniyu_bus bumped from 0.4.1 to 0.4.2

## [0.4.1](https://github.com/puniyu/puniyu/compare/adapter-v0.4.0...adapter-v0.4.1) (2025-11-16)


### 🔧 其他更新

* **adapter:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_macros bumped from 0.4.0 to 0.4.1
    * puniyu_bot bumped from 0.4.0 to 0.4.1
    * puniyu_element bumped from 0.4.0 to 0.4.1
    * puniyu_event bumped from 0.4.0 to 0.4.1
    * puniyu_event_bus bumped from 0.4.0 to 0.4.1
    * puniyu_common bumped from 0.4.0 to 0.4.1
    * puniyu_builder bumped from 0.4.0 to 0.4.1
    * puniyu_contact bumped from 0.4.0 to 0.4.1
    * puniyu_sender bumped from 0.4.0 to 0.4.1
    * puniyu_adapter_api bumped from 0.4.0 to 0.4.1

## [0.4.0](https://github.com/puniyu/puniyu/compare/adapter-v0.2.2...adapter-v0.4.0) (2025-11-16)


### 🔧 其他更新

* **adapter:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_macros bumped from 0.3.0 to 0.4.0
    * puniyu_bot bumped from 0.3.0 to 0.4.0
    * puniyu_element bumped from 0.3.0 to 0.4.0
    * puniyu_event bumped from 0.3.0 to 0.4.0
    * puniyu_event_bus bumped from 0.3.0 to 0.4.0
    * puniyu_common bumped from 0.3.0 to 0.4.0
    * puniyu_builder bumped from 0.3.0 to 0.4.0
    * puniyu_contact bumped from 0.1.10 to 0.4.0
    * puniyu_sender bumped from 0.1.10 to 0.4.0
    * puniyu_adapter_api bumped from 0.2.1 to 0.4.0

## [0.2.2](https://github.com/puniyu/puniyu/compare/adapter-v0.2.1...adapter-v0.2.2) (2025-11-15)


### 🔧 其他更新

* **release:** 调整发布配置并更新依赖版本 ([381ca55](https://github.com/puniyu/puniyu/commit/381ca558c7ca6b4bcc9b9386ecc228e8679f7305))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_macros bumped from 0.2.0 to 0.3.0
    * puniyu_bot bumped from 0.2.0 to 0.3.0
    * puniyu_element bumped from 0.2.0 to 0.3.0
    * puniyu_event bumped from 0.2.0 to 0.3.0
    * puniyu_event_bus bumped from 0.2.0 to 0.3.0
    * puniyu_common bumped from 0.2.0 to 0.3.0
    * puniyu_builder bumped from 0.2.0 to 0.3.0
    * puniyu_contact bumped from 0.1.9 to 0.1.10
    * puniyu_sender bumped from 0.1.9 to 0.1.10
    * puniyu_adapter_api bumped from 0.2.0 to 0.2.1

## [0.2.1](https://github.com/puniyu/puniyu/compare/adapter-v0.2.0...adapter-v0.2.1) (2025-11-15)


### ♻️ 代码重构

* **project:** 重构项目结构 ([b1b389f](https://github.com/puniyu/puniyu/commit/b1b389f25dae5899b49133be88ba348930117972))
