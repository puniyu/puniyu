# Changelog

## [0.4.2](https://github.com/puniyu/puniyu/compare/common-v0.4.1...common-v0.4.2) (2025-11-22)


### 🔧 其他更新

* **deps:** update actions/checkout action to v5.0.1 ([1c68c9c](https://github.com/puniyu/puniyu/commit/1c68c9c096e16d77d1c632e8f6f4b65191878d33))


### ♻️ 代码重构

* **config:** 重构配置文件错误处理和TOML操作函数 ([a18e37b](https://github.com/puniyu/puniyu/commit/a18e37b5be5fcf8b64cd461eb21dffdaa7807aab))
* **error:** 统一错误处理机制 ([586272d](https://github.com/puniyu/puniyu/commit/586272ddf8828cfa7af809bc01de95d1245c4d9b))
* 重构工作区crates ([#53](https://github.com/puniyu/puniyu/issues/53)) ([f55ab51](https://github.com/puniyu/puniyu/commit/f55ab519b9275c19773f16acfddff0c44db8e48c))

## [0.4.1](https://github.com/puniyu/puniyu/compare/common-v0.4.0...common-v0.4.1) (2025-11-16)


### 🐛 错误修复

* **path:** 重构工作目录设置逻辑 ([df51eac](https://github.com/puniyu/puniyu/commit/df51eac9e4fe92b11df2867ebdeca78fe62b2022))

## [0.4.0](https://github.com/puniyu/puniyu/compare/common-v0.3.0...common-v0.4.0) (2025-11-16)


### ✨ 新功能

* **adapter:** 初始化适配器数据目录结构 ([0e7413b](https://github.com/puniyu/puniyu/commit/0e7413b8790cb2b6c7f1bf7ed43046be1169cfaf))
* **core:** 自动创建插件和适配器数据目录 ([881a42e](https://github.com/puniyu/puniyu/commit/881a42ece6fb13ae8ad11c94e01e9c4463a32ec4))


### 🔧 其他更新

* **deps:** update puniyu core packages ([be007a5](https://github.com/puniyu/puniyu/commit/be007a5b1cb9cb7e2b640905bdaa390adb14d319))
* release main ([b22b2f0](https://github.com/puniyu/puniyu/commit/b22b2f017c88290346428c229c975cc570bc70d1))
* release main ([be9bdfe](https://github.com/puniyu/puniyu/commit/be9bdfe1fcee37185d800f1cf0bd5758d83776e1))


### ♻️ 代码重构

* **adapter:** 重构适配器模块结构和依赖关系 ([1cedfac](https://github.com/puniyu/puniyu/commit/1cedfac70a93d071b25ea2721df7c9f41123e1bf))
* **config:** 将日志路径类型从 String 改为 PathBuf ([0cc4759](https://github.com/puniyu/puniyu/commit/0cc4759a97b7c4aece6818171f2044ecd554e7be))
* **core:** 重构应用初始化逻辑 ([e7755e5](https://github.com/puniyu/puniyu/commit/e7755e5362ffa318fd38abfd30a00e5f8b25d43e))
* **core:** 重构消息事件和适配器API错误类型 ([e0aeeb1](https://github.com/puniyu/puniyu/commit/e0aeeb19fdff296beece58fb1cc5d8ebd36abf26))

## [0.3.0](https://github.com/puniyu/puniyu/compare/v0.2.0...v0.3.0) (2025-11-15)


### ✨ 新功能

* **adapter:** 初始化适配器数据目录结构 ([0e7413b](https://github.com/puniyu/puniyu/commit/0e7413b8790cb2b6c7f1bf7ed43046be1169cfaf))
* **core:** 自动创建插件和适配器数据目录 ([881a42e](https://github.com/puniyu/puniyu/commit/881a42ece6fb13ae8ad11c94e01e9c4463a32ec4))


### 🔧 其他更新

* **deps:** update puniyu core packages ([be007a5](https://github.com/puniyu/puniyu/commit/be007a5b1cb9cb7e2b640905bdaa390adb14d319))
* release main ([be9bdfe](https://github.com/puniyu/puniyu/commit/be9bdfe1fcee37185d800f1cf0bd5758d83776e1))


### ♻️ 代码重构

* **adapter:** 重构适配器模块结构和依赖关系 ([1cedfac](https://github.com/puniyu/puniyu/commit/1cedfac70a93d071b25ea2721df7c9f41123e1bf))
* **config:** 将日志路径类型从 String 改为 PathBuf ([0cc4759](https://github.com/puniyu/puniyu/commit/0cc4759a97b7c4aece6818171f2044ecd554e7be))
* **core:** 重构应用初始化逻辑 ([e7755e5](https://github.com/puniyu/puniyu/commit/e7755e5362ffa318fd38abfd30a00e5f8b25d43e))
* **core:** 重构消息事件和适配器API错误类型 ([e0aeeb1](https://github.com/puniyu/puniyu/commit/e0aeeb19fdff296beece58fb1cc5d8ebd36abf26))

## [0.2.0](https://github.com/puniyu/puniyu/compare/common-v0.1.8...common-v0.2.0) (2025-11-15)


### ✨ 新功能

* **adapter:** 初始化适配器数据目录结构 ([0e7413b](https://github.com/puniyu/puniyu/commit/0e7413b8790cb2b6c7f1bf7ed43046be1169cfaf))
* **core:** 自动创建插件和适配器数据目录 ([881a42e](https://github.com/puniyu/puniyu/commit/881a42ece6fb13ae8ad11c94e01e9c4463a32ec4))


### 🔧 其他更新

* **deps:** update puniyu core packages ([be007a5](https://github.com/puniyu/puniyu/commit/be007a5b1cb9cb7e2b640905bdaa390adb14d319))


### ♻️ 代码重构

* **adapter:** 重构适配器模块结构和依赖关系 ([1cedfac](https://github.com/puniyu/puniyu/commit/1cedfac70a93d071b25ea2721df7c9f41123e1bf))
* **config:** 将日志路径类型从 String 改为 PathBuf ([0cc4759](https://github.com/puniyu/puniyu/commit/0cc4759a97b7c4aece6818171f2044ecd554e7be))
* **core:** 重构应用初始化逻辑 ([e7755e5](https://github.com/puniyu/puniyu/commit/e7755e5362ffa318fd38abfd30a00e5f8b25d43e))
* **core:** 重构消息事件和适配器API错误类型 ([e0aeeb1](https://github.com/puniyu/puniyu/commit/e0aeeb19fdff296beece58fb1cc5d8ebd36abf26))
