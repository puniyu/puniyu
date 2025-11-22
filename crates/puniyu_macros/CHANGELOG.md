# Changelog

## [0.4.2](https://github.com/puniyu/puniyu/compare/macros-v0.4.1...macros-v0.4.2) (2025-11-22)


### 🔧 其他更新

* **deps:** update actions/checkout action to v5.0.1 ([1c68c9c](https://github.com/puniyu/puniyu/commit/1c68c9c096e16d77d1c632e8f6f4b65191878d33))


### ♻️ 代码重构

* **error:** 统一错误处理机制 ([586272d](https://github.com/puniyu/puniyu/commit/586272ddf8828cfa7af809bc01de95d1245c4d9b))
* **macro:** 移除冗余的插件名称获取逻辑 ([7c66203](https://github.com/puniyu/puniyu/commit/7c6620385ee259a1f32e142d4b4d18344ff008b7))
* **workspace:** 重构项目结构和依赖管理 ([520087e](https://github.com/puniyu/puniyu/commit/520087e38b49ff9e3aabe04be946038b81114b4d))
* 重构工作区crates ([#53](https://github.com/puniyu/puniyu/issues/53)) ([f55ab51](https://github.com/puniyu/puniyu/commit/f55ab519b9275c19773f16acfddff0c44db8e48c))

## [0.4.1](https://github.com/puniyu/puniyu/compare/macros-v0.4.0...macros-v0.4.1) (2025-11-16)


### 🔧 其他更新

* **macros:** Synchronize puniyu versions

## [0.4.0](https://github.com/puniyu/puniyu/compare/macros-v0.3.0...macros-v0.4.0) (2025-11-16)


### ✨ 新功能

* **adapter:** 导出适配器相关类型和构建器 ([73e9e76](https://github.com/puniyu/puniyu/commit/73e9e764603f1c2fb493a86da9c5d815fb95e55e))
* **adapter:** 重构适配器宏并移除日志宏定义 ([314341c](https://github.com/puniyu/puniyu/commit/314341cc4b3c530eed489b42e5e8a721dc6293f7))
* **core:** 自动创建插件和适配器数据目录 ([881a42e](https://github.com/puniyu/puniyu/commit/881a42ece6fb13ae8ad11c94e01e9c4463a32ec4))
* **macro:** 为任务宏添加 name 参数支持 ([011f7a1](https://github.com/puniyu/puniyu/commit/011f7a109ba9650372a04cc977696c07aa599350))
* **server:** 添加服务器停止功能并优化启动逻辑 ([7324370](https://github.com/puniyu/puniyu/commit/7324370202ead7e9902c56964ebf33d683ed136b))


### 🔧 其他更新

* release main ([b22b2f0](https://github.com/puniyu/puniyu/commit/b22b2f017c88290346428c229c975cc570bc70d1))
* release main ([be9bdfe](https://github.com/puniyu/puniyu/commit/be9bdfe1fcee37185d800f1cf0bd5758d83776e1))


### ♻️ 代码重构

* **adapter:** 简化适配器加载逻辑并移除ABI版本检查 ([30104ed](https://github.com/puniyu/puniyu/commit/30104edcd5c1e81ffb87a4da6718bbc0399ff941))
* **adapter:** 重构适配器模块结构和依赖关系 ([1cedfac](https://github.com/puniyu/puniyu/commit/1cedfac70a93d071b25ea2721df7c9f41123e1bf))
* **event:** 引入统一事件上下文和通知/请求事件 ([#31](https://github.com/puniyu/puniyu/issues/31)) ([05bd056](https://github.com/puniyu/puniyu/commit/05bd05690b3fa47443f6d5982c799f88cd900f54))

## [0.3.0](https://github.com/puniyu/puniyu/compare/v0.2.0...v0.3.0) (2025-11-15)


### ✨ 新功能

* **adapter:** 导出适配器相关类型和构建器 ([73e9e76](https://github.com/puniyu/puniyu/commit/73e9e764603f1c2fb493a86da9c5d815fb95e55e))
* **adapter:** 重构适配器宏并移除日志宏定义 ([314341c](https://github.com/puniyu/puniyu/commit/314341cc4b3c530eed489b42e5e8a721dc6293f7))
* **core:** 自动创建插件和适配器数据目录 ([881a42e](https://github.com/puniyu/puniyu/commit/881a42ece6fb13ae8ad11c94e01e9c4463a32ec4))
* **macro:** 为任务宏添加 name 参数支持 ([011f7a1](https://github.com/puniyu/puniyu/commit/011f7a109ba9650372a04cc977696c07aa599350))
* **server:** 添加服务器停止功能并优化启动逻辑 ([7324370](https://github.com/puniyu/puniyu/commit/7324370202ead7e9902c56964ebf33d683ed136b))


### 🔧 其他更新

* release main ([be9bdfe](https://github.com/puniyu/puniyu/commit/be9bdfe1fcee37185d800f1cf0bd5758d83776e1))


### ♻️ 代码重构

* **adapter:** 简化适配器加载逻辑并移除ABI版本检查 ([30104ed](https://github.com/puniyu/puniyu/commit/30104edcd5c1e81ffb87a4da6718bbc0399ff941))
* **adapter:** 重构适配器模块结构和依赖关系 ([1cedfac](https://github.com/puniyu/puniyu/commit/1cedfac70a93d071b25ea2721df7c9f41123e1bf))
* **event:** 引入统一事件上下文和通知/请求事件 ([#31](https://github.com/puniyu/puniyu/issues/31)) ([05bd056](https://github.com/puniyu/puniyu/commit/05bd05690b3fa47443f6d5982c799f88cd900f54))

## [0.2.0](https://github.com/puniyu/puniyu/compare/macros-v0.1.8...macros-v0.2.0) (2025-11-15)


### ✨ 新功能

* **adapter:** 导出适配器相关类型和构建器 ([73e9e76](https://github.com/puniyu/puniyu/commit/73e9e764603f1c2fb493a86da9c5d815fb95e55e))
* **adapter:** 重构适配器宏并移除日志宏定义 ([314341c](https://github.com/puniyu/puniyu/commit/314341cc4b3c530eed489b42e5e8a721dc6293f7))
* **core:** 自动创建插件和适配器数据目录 ([881a42e](https://github.com/puniyu/puniyu/commit/881a42ece6fb13ae8ad11c94e01e9c4463a32ec4))
* **macro:** 为任务宏添加 name 参数支持 ([011f7a1](https://github.com/puniyu/puniyu/commit/011f7a109ba9650372a04cc977696c07aa599350))
* **server:** 添加服务器停止功能并优化启动逻辑 ([7324370](https://github.com/puniyu/puniyu/commit/7324370202ead7e9902c56964ebf33d683ed136b))


### ♻️ 代码重构

* **adapter:** 简化适配器加载逻辑并移除ABI版本检查 ([30104ed](https://github.com/puniyu/puniyu/commit/30104edcd5c1e81ffb87a4da6718bbc0399ff941))
* **adapter:** 重构适配器模块结构和依赖关系 ([1cedfac](https://github.com/puniyu/puniyu/commit/1cedfac70a93d071b25ea2721df7c9f41123e1bf))
* **event:** 引入统一事件上下文和通知/请求事件 ([#31](https://github.com/puniyu/puniyu/issues/31)) ([05bd056](https://github.com/puniyu/puniyu/commit/05bd05690b3fa47443f6d5982c799f88cd900f54))
