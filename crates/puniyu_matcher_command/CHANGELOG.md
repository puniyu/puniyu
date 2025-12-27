# Changelog

## [0.7.0](https://github.com/puniyu/puniyu/compare/matcher-command-v0.6.0...matcher-command-v0.7.0) (2025-12-27)


### ✨ 新功能

* **command:** 添加命令冷却期检查功能 ([09025b7](https://github.com/puniyu/puniyu/commit/09025b72c088aa772875b75140d73d6889ec4c3a))
* **config:** 添加全局命令前缀配置并优化模块结构 ([e0e5bb5](https://github.com/puniyu/puniyu/commit/e0e5bb5ac9eb24a37189a3d50b42d3db8db58dde))
* **plugin:** 添加基础插件及状态命令功能 ([2503696](https://github.com/puniyu/puniyu/commit/2503696fa293909244f5110e8504ae00bf8b962b))
* **protocol:** 添加账户和适配器协议定义及Bot信息结构 ([#93](https://github.com/puniyu/puniyu/issues/93)) ([9611fba](https://github.com/puniyu/puniyu/commit/9611fba37182df82297bd1a4596473f9e81c2b5c))


### ⚡️ 性能优化

* **prefix:** 优化前缀使用方式 ([4952284](https://github.com/puniyu/puniyu/commit/4952284d1b1772412cb7cc1638a75303c02fda5f))


### 🎨 代码样式

* **code:** 项目格式化 ([dc3d850](https://github.com/puniyu/puniyu/commit/dc3d850bcca149de821bc5ec700ae98f567f4a79))


### 🔧 其他更新

* release main ([63f497e](https://github.com/puniyu/puniyu/commit/63f497e6284153150f15b9c17e22bd84d532415c))
* **release:** 切换到 release-plz 发布流程 ([2e2e061](https://github.com/puniyu/puniyu/commit/2e2e06162cc9d953d4bf193669a8ff8babc5e9f5))


### ♻️ 代码重构

* **cooldown:** 重构冷却期检查逻辑以支持统一作用域枚举 ([b5ae1ec](https://github.com/puniyu/puniyu/commit/b5ae1ec7fbea7f5e28e02f0aa6717bfc379979f0))
* **core:** 重构命令匹配器与上下文处理逻辑 ([3aca600](https://github.com/puniyu/puniyu/commit/3aca600a94a09079ecbd8e84cf51376fc1222a99))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_config bumped from 0.6.0 to 0.7.0
    * puniyu_registry bumped from 0.6.0 to 0.7.0
    * puniyu_types bumped from 0.6.0 to 0.7.0

## [0.6.0](https://github.com/puniyu/puniyu/compare/matcher-command-v0.5.12...matcher-command-v0.6.0) (2025-12-02)


### ✨ 新功能

* **config:** 添加全局命令前缀配置并优化模块结构 ([e0e5bb5](https://github.com/puniyu/puniyu/commit/e0e5bb5ac9eb24a37189a3d50b42d3db8db58dde))


### ⚡️ 性能优化

* **prefix:** 优化前缀使用方式 ([4952284](https://github.com/puniyu/puniyu/commit/4952284d1b1772412cb7cc1638a75303c02fda5f))


### 🔧 其他更新

* **release:** 切换到 release-plz 发布流程 ([2e2e061](https://github.com/puniyu/puniyu/commit/2e2e06162cc9d953d4bf193669a8ff8babc5e9f5))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_config bumped from 0.5.12 to 0.6.0
    * puniyu_registry bumped from 0.5.12 to 0.6.0
    * puniyu_types bumped from 0.5.12 to 0.6.0
