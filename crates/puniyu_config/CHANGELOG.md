# 变更日志

## [0.5.12](https://github.com/puniyu/puniyu/compare/config-v0.5.11...config-v0.5.12) (2025-11-24)


### 🔧 其他更新

* **config:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.11 to 0.5.12

## [0.5.11](https://github.com/puniyu/puniyu/compare/config-v0.5.10...config-v0.5.11) (2025-11-23)


### 🔧 其他更新

* **config:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.10 to 0.5.11

## [0.5.10](https://github.com/puniyu/puniyu/compare/config-v0.5.9...config-v0.5.10) (2025-11-23)


### 🔧 其他更新

* **config:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.9 to 0.5.10

## [0.5.9](https://github.com/puniyu/puniyu/compare/config-v0.5.8...config-v0.5.9) (2025-11-23)


### 🔧 其他更新

* **config:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.8 to 0.5.9

## [0.5.8](https://github.com/puniyu/puniyu/compare/config-v0.5.7...config-v0.5.8) (2025-11-23)


### 🔧 其他更新

* **config:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.7 to 0.5.8

## [0.5.7](https://github.com/puniyu/puniyu/compare/config-v0.5.6...config-v0.5.7) (2025-11-23)


### 🔧 其他更新

* **config:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.6 to 0.5.7

## [0.5.6](https://github.com/puniyu/puniyu/compare/config-v0.5.4...config-v0.5.6) (2025-11-23)


### 🔧 其他更新

* **config:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.4 to 0.5.6

## [0.5.4](https://github.com/puniyu/puniyu/compare/config-v0.5.1...config-v0.5.4) (2025-11-23)


### 🔧 其他更新

* **config:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.1 to 0.5.4

## [0.5.1](https://github.com/puniyu/puniyu/compare/config-v0.5.0...config-v0.5.1) (2025-11-23)


### 🔧 其他更新

* **config:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.0 to 0.5.1

## [0.5.0](https://github.com/puniyu/puniyu/compare/config-v0.4.1...config-v0.5.0) (2025-11-23)


### ✨ 新功能

* **config:** 将配置序列化格式从 JSON 切换为 TOML ([48fc976](https://github.com/puniyu/puniyu/commit/48fc976274386311b62fa1d344807a68432b99f0))
* **config:** 重构配置管理模块以支持动态注册与热重载 ([f3234c1](https://github.com/puniyu/puniyu/commit/f3234c16ea7d49b4cae2cdd0bda024f390778497))


### ♻️ 代码重构

* 重构工作区crates ([#53](https://github.com/puniyu/puniyu/issues/53)) ([f55ab51](https://github.com/puniyu/puniyu/commit/f55ab519b9275c19773f16acfddff0c44db8e48c))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.4.1 to 0.5.0

## [0.4.1](https://github.com/puniyu/puniyu/compare/config-v0.4.0...config-v0.4.1) (2025-11-16)


### 🔧 其他更新

* **config:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.4.0 to 0.4.1

## [0.4.0](https://github.com/puniyu/puniyu/compare/config-v0.3.0...config-v0.4.0) (2025-11-16)


### ✨ 新功能

* **config:** 将配置模块拆分为独立的puniyu_config crate ([10351d6](https://github.com/puniyu/puniyu/commit/10351d6451cd53bcc35dadbeb885553cfb3d66d3))
* **core:** 自动创建插件和适配器数据目录 ([881a42e](https://github.com/puniyu/puniyu/commit/881a42ece6fb13ae8ad11c94e01e9c4463a32ec4))


### 🔧 其他更新

* **config:** 初始化配置模块并添加 README 文档 ([e9099fc](https://github.com/puniyu/puniyu/commit/e9099fc66da3b14413f3851c82771f148cff5c10))
* release main ([b22b2f0](https://github.com/puniyu/puniyu/commit/b22b2f017c88290346428c229c975cc570bc70d1))
* release main ([be9bdfe](https://github.com/puniyu/puniyu/commit/be9bdfe1fcee37185d800f1cf0bd5758d83776e1))
* **release:** 调整发布配置并更新依赖版本 ([381ca55](https://github.com/puniyu/puniyu/commit/381ca558c7ca6b4bcc9b9386ecc228e8679f7305))


### ♻️ 代码重构

* **adapter:** 简化适配器加载逻辑并移除ABI版本检查 ([30104ed](https://github.com/puniyu/puniyu/commit/30104edcd5c1e81ffb87a4da6718bbc0399ff941))
* **config:** 将日志路径类型从 String 改为 PathBuf ([0cc4759](https://github.com/puniyu/puniyu/commit/0cc4759a97b7c4aece6818171f2044ecd554e7be))
* **config:** 重构环境变量初始化逻辑 ([3c81fa6](https://github.com/puniyu/puniyu/commit/3c81fa683631a8dd5364e1228fc6d8004e5ba5f3))
* **config:** 重构配置文件监听器实现 ([4c99137](https://github.com/puniyu/puniyu/commit/4c9913784f5a40bcb8d13494121489ea86ce17c4))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.3.0 to 0.4.0

## [0.3.0](https://github.com/puniyu/puniyu/compare/v0.2.0...v0.3.0) (2025-11-15)


### ✨ 新功能

* **config:** 将配置模块拆分为独立的puniyu_config crate ([10351d6](https://github.com/puniyu/puniyu/commit/10351d6451cd53bcc35dadbeb885553cfb3d66d3))
* **core:** 自动创建插件和适配器数据目录 ([881a42e](https://github.com/puniyu/puniyu/commit/881a42ece6fb13ae8ad11c94e01e9c4463a32ec4))


### 🔧 其他更新

* **config:** 初始化配置模块并添加 README 文档 ([e9099fc](https://github.com/puniyu/puniyu/commit/e9099fc66da3b14413f3851c82771f148cff5c10))
* release main ([be9bdfe](https://github.com/puniyu/puniyu/commit/be9bdfe1fcee37185d800f1cf0bd5758d83776e1))
* **release:** 调整发布配置并更新依赖版本 ([381ca55](https://github.com/puniyu/puniyu/commit/381ca558c7ca6b4bcc9b9386ecc228e8679f7305))


### ♻️ 代码重构

* **adapter:** 简化适配器加载逻辑并移除ABI版本检查 ([30104ed](https://github.com/puniyu/puniyu/commit/30104edcd5c1e81ffb87a4da6718bbc0399ff941))
* **config:** 将日志路径类型从 String 改为 PathBuf ([0cc4759](https://github.com/puniyu/puniyu/commit/0cc4759a97b7c4aece6818171f2044ecd554e7be))
* **config:** 重构环境变量初始化逻辑 ([3c81fa6](https://github.com/puniyu/puniyu/commit/3c81fa683631a8dd5364e1228fc6d8004e5ba5f3))
* **config:** 重构配置文件监听器实现 ([4c99137](https://github.com/puniyu/puniyu/commit/4c9913784f5a40bcb8d13494121489ea86ce17c4))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.2.0 to 0.3.0

## [0.2.0](https://github.com/puniyu/puniyu/compare/config-v0.1.8...config-v0.2.0) (2025-11-15)


### ✨ 新功能

* **config:** 将配置模块拆分为独立的puniyu_config crate ([10351d6](https://github.com/puniyu/puniyu/commit/10351d6451cd53bcc35dadbeb885553cfb3d66d3))
* **core:** 自动创建插件和适配器数据目录 ([881a42e](https://github.com/puniyu/puniyu/commit/881a42ece6fb13ae8ad11c94e01e9c4463a32ec4))


### 🔧 其他更新

* **config:** 初始化配置模块并添加 README 文档 ([e9099fc](https://github.com/puniyu/puniyu/commit/e9099fc66da3b14413f3851c82771f148cff5c10))


### ♻️ 代码重构

* **adapter:** 简化适配器加载逻辑并移除ABI版本检查 ([30104ed](https://github.com/puniyu/puniyu/commit/30104edcd5c1e81ffb87a4da6718bbc0399ff941))
* **config:** 将日志路径类型从 String 改为 PathBuf ([0cc4759](https://github.com/puniyu/puniyu/commit/0cc4759a97b7c4aece6818171f2044ecd554e7be))
* **config:** 重构环境变量初始化逻辑 ([3c81fa6](https://github.com/puniyu/puniyu/commit/3c81fa683631a8dd5364e1228fc6d8004e5ba5f3))
* **config:** 重构配置文件监听器实现 ([4c99137](https://github.com/puniyu/puniyu/commit/4c9913784f5a40bcb8d13494121489ea86ce17c4))
