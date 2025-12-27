# 变更日志

## [0.7.0](https://github.com/puniyu/puniyu/compare/macros-v0.6.0...macros-v0.7.0) (2025-12-27)


### ✨ 新功能

* **command:** 添加命令权限控制功能 ([cc0013a](https://github.com/puniyu/puniyu/commit/cc0013aff04d8efea0b9cdda3f11eae4d1eac97b))
* **plugin:** 新增服务端插件支持 ([7f15acf](https://github.com/puniyu/puniyu/commit/7f15acf148d002e33ef246b3a65a08866a44393f))
* **protocol:** 添加账户和适配器协议定义及Bot信息结构 ([#93](https://github.com/puniyu/puniyu/issues/93)) ([9611fba](https://github.com/puniyu/puniyu/commit/9611fba37182df82297bd1a4596473f9e81c2b5c))


### 🎨 代码样式

* **code:** 项目格式化 ([dc3d850](https://github.com/puniyu/puniyu/commit/dc3d850bcca149de821bc5ec700ae98f567f4a79))


### ♻️ 代码重构

* **adapter:** 重构适配器信息结构与初始化逻辑 ([2e45256](https://github.com/puniyu/puniyu/commit/2e4525633031ec401f058507218cb2731ac24479))

## [0.6.0](https://github.com/puniyu/puniyu/compare/macros-v0.5.12...macros-v0.6.0) (2025-12-02)


### ✨ 新功能

* **config:** 添加全局命令前缀配置并优化模块结构 ([e0e5bb5](https://github.com/puniyu/puniyu/commit/e0e5bb5ac9eb24a37189a3d50b42d3db8db58dde))
* **macro:** 支持对象格式的命令参数定义 ([672dddd](https://github.com/puniyu/puniyu/commit/672ddddd276eb24f0572bd5a748d01c9a4ae64af))
* **macro:** 重构宏系统并增强命令参数支持 ([e8f3c23](https://github.com/puniyu/puniyu/commit/e8f3c23e42cd94fb567dce569cbe4477014300b1))
* **plugin:** 支持命令参数的位置和命名模式 ([85e92d4](https://github.com/puniyu/puniyu/commit/85e92d4ec50367ad3d1e1194ee1542ce74dd82dd))
* **puniyu_macros:** 支持对象格式参数定义 ([3a8a2d6](https://github.com/puniyu/puniyu/commit/3a8a2d67b6728079a76594e39d6938390d7b3612))


### 🐛 错误修复

* **command:** 优化命令行参数错误提示信息 ([3f66a17](https://github.com/puniyu/puniyu/commit/3f66a17c3ec4fa43641f7fe92d31e794b088926a))
* **puniyu_plugin:** 修正plugin宏获取crate name错误 ([b8fdbd7](https://github.com/puniyu/puniyu/commit/b8fdbd7b16371cdcbdee23d90c3075aff4cc4ee4))
* **puniyu_plugin:** 添加缺少的导入 ([23f7f8a](https://github.com/puniyu/puniyu/commit/23f7f8a459f941971a203063d6215c9779b74411))


### ♻️ 代码重构

* **command:** 修正日志格式 ([b8fdbd7](https://github.com/puniyu/puniyu/commit/b8fdbd7b16371cdcbdee23d90c3075aff4cc4ee4))
* **core:** 重新组织模块导入顺序并清理无用导出 ([8da3556](https://github.com/puniyu/puniyu/commit/8da35564bb6e99777d30d5c48497e11358c51915))

## [0.5.12](https://github.com/puniyu/puniyu/compare/macros-v0.5.11...macros-v0.5.12) (2025-11-24)


### 🔧 其他更新

* **macros:** Synchronize puniyu versions

## [0.5.11](https://github.com/puniyu/puniyu/compare/macros-v0.5.10...macros-v0.5.11) (2025-11-23)


### ♻️ 代码重构

* **macro:** 更新宏定义中的模块路径引用 ([46afc41](https://github.com/puniyu/puniyu/commit/46afc41f42dc44febb072f8372fa7be5dc24d082))

## [0.5.10](https://github.com/puniyu/puniyu/compare/macros-v0.5.9...macros-v0.5.10) (2025-11-23)


### ♻️ 代码重构

* **command:** 重构命令处理结果类型和参数验证 ([58d4eeb](https://github.com/puniyu/puniyu/commit/58d4eebb41cacabc7663b40a93181b789feb1e0a))

## [0.5.9](https://github.com/puniyu/puniyu/compare/macros-v0.5.8...macros-v0.5.9) (2025-11-23)


### 🐛 错误修复

* **plugin:** 修复缺少的导入 ([69e01e4](https://github.com/puniyu/puniyu/commit/69e01e4d80ae0e0c47e2d7e2d27ce24de70ae227))

## [0.5.8](https://github.com/puniyu/puniyu/compare/macros-v0.5.7...macros-v0.5.8) (2025-11-23)


### 🔧 其他更新

* **macros:** Synchronize puniyu versions

## [0.5.7](https://github.com/puniyu/puniyu/compare/macros-v0.5.6...macros-v0.5.7) (2025-11-23)


### 🔧 其他更新

* **macros:** Synchronize puniyu versions

## [0.5.6](https://github.com/puniyu/puniyu/compare/macros-v0.5.4...macros-v0.5.6) (2025-11-23)


### 🔧 其他更新

* **macros:** Synchronize puniyu versions

## [0.5.4](https://github.com/puniyu/puniyu/compare/macros-v0.5.1...macros-v0.5.4) (2025-11-23)


### 🔧 其他更新

* **macros:** Synchronize puniyu versions

## [0.5.1](https://github.com/puniyu/puniyu/compare/macros-v0.5.0...macros-v0.5.1) (2025-11-23)


### 🔧 其他更新

* **macros:** Synchronize puniyu versions

## [0.5.0](https://github.com/puniyu/puniyu/compare/macros-v0.4.1...macros-v0.5.0) (2025-11-23)


### ✨ 新功能

* **adapter:** 支持配置文件读取功能 ([a9fc6e2](https://github.com/puniyu/puniyu/commit/a9fc6e2aed53370db0c78a0035c37eec53114445))
* **config:** 将配置序列化格式从 JSON 切换为 TOML ([48fc976](https://github.com/puniyu/puniyu/commit/48fc976274386311b62fa1d344807a68432b99f0))
* **config:** 重构配置管理模块以支持动态注册与热重载 ([f3234c1](https://github.com/puniyu/puniyu/commit/f3234c16ea7d49b4cae2cdd0bda024f390778497))
* **plugin:** 添加插件配置支持 ([dc7d1eb](https://github.com/puniyu/puniyu/commit/dc7d1ebcf2245f53f3a58b203edd405aa7cc8c1c))


### 🔧 其他更新

* **deps:** update actions/checkout action to v5.0.1 ([1c68c9c](https://github.com/puniyu/puniyu/commit/1c68c9c096e16d77d1c632e8f6f4b65191878d33))


### ♻️ 代码重构

* **config:** 添加适配器配置系统支持 ([5358888](https://github.com/puniyu/puniyu/commit/5358888a950988a6be04c9b539bdac95c9b09b8d))
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
