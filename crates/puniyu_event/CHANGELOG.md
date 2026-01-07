# 变更日志
## [0.6.2](https://github.com/puniyu/puniyu/compare/puniyu_event-v0.6.1...puniyu_event-v0.6.2) - 2026-01-07


### ⛰️ 新功能


- *(adapter)* 实现QQ协议适配器API接口 ([#34](https://github.com/puniyu/puniyu/pull/34)) (由 @shiwuliya 提供) (#34) - ([18e4f9e](https://github.com/puniyu/puniyu/commit/18e4f9e7245cf8f1355d4f23eca0d2df42e8f7e5))
- *(core)* 自动创建插件和适配器数据目录 (由 @shiwuliya 提供) - ([881a42e](https://github.com/puniyu/puniyu/commit/881a42ece6fb13ae8ad11c94e01e9c4463a32ec4))



### 🐛 Bug 修复


- *(event)* 优化事件分发机制 (由 @shiwuliya 提供) - ([e634a3c](https://github.com/puniyu/puniyu/commit/e634a3cf25958fe96db62d4b6f56b78aaf843bb8))



### 🚜 重构


- *(adapter)* 重构适配器模块结构和依赖关系 (由 @shiwuliya 提供) - ([3c8e01f](https://github.com/puniyu/puniyu/commit/3c8e01f3bc7046220d06e4fe9c0a919a4f9d26f2))
- *(adapter)* 重构适配器API结构并添加模块化接口 ([#100](https://github.com/puniyu/puniyu/pull/100)) (由 @shiwuliya 提供) (#100) - ([6558f4f](https://github.com/puniyu/puniyu/commit/6558f4faed67a89a59c4c2790a7ba986581888a5))
- *(adapter)* 优化 Avatar 类型实现并添加服务器 logo 接口 (由 @shiwuliya 提供) - ([952c18b](https://github.com/puniyu/puniyu/commit/952c18b3008a5e31fd00127dc6d2fb55568c9796))
- *(adapter)* 简化适配器加载逻辑并移除ABI版本检查 (由 @shiwuliya 提供) (#37) - ([30104ed](https://github.com/puniyu/puniyu/commit/30104edcd5c1e81ffb87a4da6718bbc0399ff941))
- *(contact)* 重构联系人类型系统，引入 trait 抽象 (由 @shiwuliya 提供) - ([c403f08](https://github.com/puniyu/puniyu/commit/c403f08b8aaebfa071995c8179c9d75a8ae532d9))
- *(core)* 重新组织模块导入顺序并清理无用导出 (由 @shiwuliya 提供) - ([8da3556](https://github.com/puniyu/puniyu/commit/8da35564bb6e99777d30d5c48497e11358c51915))
- *(core)* 重构消息事件和适配器API错误类型 (由 @shiwuliya 提供) - ([e0aeeb1](https://github.com/puniyu/puniyu/commit/e0aeeb19fdff296beece58fb1cc5d8ebd36abf26))
- *(element)* 移除冗余的type字段并优化文件元素结构 (由 @shiwuliya 提供) - ([2e659d5](https://github.com/puniyu/puniyu/commit/2e659d59997543d1dac50f614ba847b6477ef0ab))
- *(error)* 统一错误处理机制 (由 @shiwuliya 提供) - ([586272d](https://github.com/puniyu/puniyu/commit/586272ddf8828cfa7af809bc01de95d1245c4d9b))
- *(event)* 重构事件系统并重命名事件总线为事件模块 (由 @shiwuliya 提供) (#98) - ([39093d4](https://github.com/puniyu/puniyu/commit/39093d4da202aaac142cd134d38e55e9a40b526d))
- *(event)* 引入统一事件上下文和通知/请求事件 ([#31](https://github.com/puniyu/puniyu/pull/31)) (由 @shiwuliya 提供) (#31) - ([05bd056](https://github.com/puniyu/puniyu/commit/05bd05690b3fa47443f6d5982c799f88cd900f54))

- 重构工作区crates ([#53](https://github.com/puniyu/puniyu/pull/53)) (由 @shiwuliya 提供) (#53) - ([f55ab51](https://github.com/puniyu/puniyu/commit/f55ab519b9275c19773f16acfddff0c44db8e48c))
- 重构项目配置 (由 @shiwuliya 提供) - ([883dfa4](https://github.com/puniyu/puniyu/commit/883dfa4fb525d7e5c27821026d727e3d8eda8600))


### 📚 文档


- *(readme)* 添加社区QQ群链接 (由 @shiwuliya 提供) - ([60bf1e7](https://github.com/puniyu/puniyu/commit/60bf1e788718844583f6a2e6809b930b7d262775))

- Update README.md (由 @allcontributors[bot] 提供) (#17) - ([52c8b9f](https://github.com/puniyu/puniyu/commit/52c8b9fcc37d46e92147e02fff0f2bf09b9eede2))


### ⚙️ 杂项


- *(config)* 初始化配置模块并添加 README 文档 (由 @shiwuliya 提供) - ([6109f15](https://github.com/puniyu/puniyu/commit/6109f151b73d1ad24c5237f5602aad40a7fbbba4))

- 初始化仓库 (由 @shiwuliya 提供) - ([51af186](https://github.com/puniyu/puniyu/commit/51af186b465079c274bb094dc3dc8b38ba959f15))



### 贡献者

* @puniyu[bot]
* @shiwuliya
* @allcontributors[bot]

## [0.6.1](https://github.com/puniyu/puniyu/compare/event-v0.6.0...event-v0.6.1) (2026-01-06)


### 🐛 错误修复

* **event:** 优化事件分发机制 ([e634a3c](https://github.com/puniyu/puniyu/commit/e634a3cf25958fe96db62d4b6f56b78aaf843bb8))


### ♻️ 代码重构

* **adapter:** 重构适配器API结构并添加模块化接口 ([#100](https://github.com/puniyu/puniyu/issues/100)) ([6558f4f](https://github.com/puniyu/puniyu/commit/6558f4faed67a89a59c4c2790a7ba986581888a5))
* **adapter:** 重构适配器模块结构和依赖关系 ([3c8e01f](https://github.com/puniyu/puniyu/commit/3c8e01f3bc7046220d06e4fe9c0a919a4f9d26f2))
* **event:** 重构事件系统并重命名事件总线为事件模块 ([39093d4](https://github.com/puniyu/puniyu/commit/39093d4da202aaac142cd134d38e55e9a40b526d))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_types bumped from 0.6.0 to 0.7.0
    * puniyu_command bumped from 0.6.0 to 0.7.0
    * puniyu_registry bumped from 0.6.0 to 0.7.0

## [0.6.0](https://github.com/puniyu/puniyu/compare/event-bus-v0.5.12...event-bus-v0.6.0) (2025-12-02)


### ✨ 新功能

* **config:** 添加全局命令前缀配置并优化模块结构 ([e0e5bb5](https://github.com/puniyu/puniyu/commit/e0e5bb5ac9eb24a37189a3d50b42d3db8db58dde))
* **macro:** 重构宏系统并增强命令参数支持 ([e8f3c23](https://github.com/puniyu/puniyu/commit/e8f3c23e42cd94fb567dce569cbe4477014300b1))


### ♻️ 代码重构

* **core:** 重新组织模块导入顺序并清理无用导出 ([8da3556](https://github.com/puniyu/puniyu/commit/8da35564bb6e99777d30d5c48497e11358c51915))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_types bumped from 0.5.12 to 0.6.0
    * puniyu_matcher_command bumped from 0.1.0 to 0.6.0
    * puniyu_handler_command bumped from 0.1.0 to 0.6.0

## [0.5.12](https://github.com/puniyu/puniyu/compare/event-bus-v0.5.11...event-bus-v0.5.12) (2025-11-24)


### 🔧 其他更新

* **event-bus:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_types bumped from 0.5.11 to 0.5.12
    * puniyu_registry bumped from 0.5.11 to 0.5.12

## [0.5.11](https://github.com/puniyu/puniyu/compare/event-bus-v0.5.10...event-bus-v0.5.11) (2025-11-23)


### 🔧 其他更新

* **event-bus:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_types bumped from 0.5.10 to 0.5.11
    * puniyu_registry bumped from 0.5.10 to 0.5.11

## [0.5.10](https://github.com/puniyu/puniyu/compare/event-bus-v0.5.9...event-bus-v0.5.10) (2025-11-23)


### 🔧 其他更新

* **event-bus:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_types bumped from 0.5.9 to 0.5.10
    * puniyu_registry bumped from 0.5.9 to 0.5.10

## [0.5.9](https://github.com/puniyu/puniyu/compare/event-bus-v0.5.8...event-bus-v0.5.9) (2025-11-23)


### 🔧 其他更新

* **event-bus:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_types bumped from 0.5.8 to 0.5.9
    * puniyu_registry bumped from 0.5.8 to 0.5.9

## [0.5.8](https://github.com/puniyu/puniyu/compare/event-bus-v0.4.10...event-bus-v0.5.8) (2025-11-23)


### 🔧 其他更新

* **event-bus:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_types bumped from 0.5.7 to 0.5.8
    * puniyu_registry bumped from 0.5.7 to 0.5.8

## [0.4.10](https://github.com/puniyu/puniyu/compare/event-bus-v0.4.9...event-bus-v0.4.10) (2025-11-23)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_types bumped from 0.5.6 to 0.5.7
    * puniyu_registry bumped from 0.5.6 to 0.5.7

## [0.4.9](https://github.com/puniyu/puniyu/compare/event-bus-v0.4.8...event-bus-v0.4.9) (2025-11-23)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_types bumped from 0.5.4 to 0.5.6
    * puniyu_registry bumped from 0.5.5 to 0.5.6

## [0.4.8](https://github.com/puniyu/puniyu/compare/event-bus-v0.4.7...event-bus-v0.4.8) (2025-11-23)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_registry bumped from 0.5.4 to 0.5.5

## [0.4.7](https://github.com/puniyu/puniyu/compare/event-bus-v0.4.6...event-bus-v0.4.7) (2025-11-23)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_registry bumped from 0.5.3 to 0.5.4

## [0.4.6](https://github.com/puniyu/puniyu/compare/event-bus-v0.4.5...event-bus-v0.4.6) (2025-11-23)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_types bumped from 0.5.1 to 0.5.4
    * puniyu_registry bumped from 0.5.2 to 0.5.3

## [0.4.5](https://github.com/puniyu/puniyu/compare/event-bus-v0.4.4...event-bus-v0.4.5) (2025-11-23)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_registry bumped from 0.5.1 to 0.5.2

## [0.4.4](https://github.com/puniyu/puniyu/compare/event-bus-v0.4.3...event-bus-v0.4.4) (2025-11-23)


### 🔧 其他更新

* **puniyu_bus:** 添加包描述信息 ([f627108](https://github.com/puniyu/puniyu/commit/f6271084dd478c2709b3bedfd1c9eb5a3b7db67e))

## [0.4.3](https://github.com/puniyu/puniyu/compare/event-bus-v0.4.2...event-bus-v0.4.3) (2025-11-23)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_types bumped from 0.5.0 to 0.5.1
    * puniyu_registry bumped from 0.5.0 to 0.5.1

## [0.4.2](https://github.com/puniyu/puniyu/compare/event-bus-v0.4.1...event-bus-v0.4.2) (2025-11-23)


### ♻️ 代码重构

* 重构工作区crates ([#53](https://github.com/puniyu/puniyu/issues/53)) ([f55ab51](https://github.com/puniyu/puniyu/commit/f55ab519b9275c19773f16acfddff0c44db8e48c))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_types bumped from 0.4.1 to 0.5.0
    * puniyu_registry bumped from 0.4.1 to 0.5.0
