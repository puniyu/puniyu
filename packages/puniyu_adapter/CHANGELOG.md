# 变更日志

## [0.7.9](https://github.com/puniyu/puniyu/compare/puniyu_adapter-v0.7.8...puniyu_adapter-v0.7.9)

### ⚙️ 杂项


- Updated the following local packages: puniyu_common, puniyu_types, puniyu_registry, puniyu_macros, puniyu_config - ([0000000](https://github.com/puniyu/puniyu/commit/0000000))

## [0.7.7](https://github.com/puniyu/puniyu/compare/puniyu_adapter-v0.7.6...puniyu_adapter-v0.7.7) - 2026-01-11


### ⚙️ 杂项



- Updated the following local packages: puniyu_types, puniyu_registry - ([0000000](https://github.com/puniyu/puniyu/commit/0000000))

## [0.7.6](https://github.com/puniyu/puniyu/compare/puniyu_adapter-v0.7.5...puniyu_adapter-v0.7.6) - 2026-01-11


### ⛰️ 新功能


- *(adapter)* 实现适配器服务器的消息转发功能 (由 @wuliya336 提供) - ([e240974](https://github.com/puniyu/puniyu/commit/e2409749a42e40eeff725bb98d3c5d987d324320))

- Hook功能实现 ([#112](https://github.com/puniyu/puniyu/pull/112)) (由 @wuliya336 提供) (#112) - ([1e7bbf7](https://github.com/puniyu/puniyu/commit/1e7bbf7b6d25fcfb5c8fcedc6a68f29ff6b8c12e))


### 🚜 重构


- *(command)* 重构命令处理器实现 (由 @wuliya336 提供) - ([a3addfd](https://github.com/puniyu/puniyu/commit/a3addfded2c550d183c5e1fcb63d6014eeb97ea9))
- *(macros)* 重构proc-macro宏 ([#119](https://github.com/puniyu/puniyu/pull/119)) (由 @wuliya336 提供) (#119) - ([9942a1b](https://github.com/puniyu/puniyu/commit/9942a1baa348377af978d9224510316a0595a5f3))




### 贡献者

* @puniyu[bot]
* @wuliya336
## [0.7.5](https://github.com/puniyu/puniyu/compare/puniyu_adapter-v0.7.4...puniyu_adapter-v0.7.5) - 2026-01-11


### ⛰️ 新功能


- *(adapter)* 实现适配器服务器的消息转发功能 (由 @wuliya336 提供) - ([e240974](https://github.com/puniyu/puniyu/commit/e2409749a42e40eeff725bb98d3c5d987d324320))

- Hook功能实现 ([#112](https://github.com/puniyu/puniyu/pull/112)) (由 @wuliya336 提供) (#112) - ([1e7bbf7](https://github.com/puniyu/puniyu/commit/1e7bbf7b6d25fcfb5c8fcedc6a68f29ff6b8c12e))


### 🚜 重构


- *(command)* 重构命令处理器实现 (由 @wuliya336 提供) - ([a3addfd](https://github.com/puniyu/puniyu/commit/a3addfded2c550d183c5e1fcb63d6014eeb97ea9))
- *(macros)* 重构proc-macro宏 ([#119](https://github.com/puniyu/puniyu/pull/119)) (由 @wuliya336 提供) (#119) - ([9942a1b](https://github.com/puniyu/puniyu/commit/9942a1baa348377af978d9224510316a0595a5f3))




### 贡献者

* @wuliya336
## [0.7.4](https://github.com/puniyu/puniyu/compare/puniyu_adapter-v0.7.3...puniyu_adapter-v0.7.4) - 2026-01-07


### ⛰️ 新功能


- *(adapter)* 实现server适配器 ([#99](https://github.com/puniyu/puniyu/pull/99)) (由 @shiwuliya 提供) (#99) - ([e1671ad](https://github.com/puniyu/puniyu/commit/e1671ad8ebc180f066be1ec58a508194d66e4850))
- *(adapter)* 支持配置文件读取功能 (由 @shiwuliya 提供) - ([a9fc6e2](https://github.com/puniyu/puniyu/commit/a9fc6e2aed53370db0c78a0035c37eec53114445))
- *(config)* 重构配置管理模块以支持动态注册与热重载 (由 @shiwuliya 提供) - ([f3234c1](https://github.com/puniyu/puniyu/commit/f3234c16ea7d49b4cae2cdd0bda024f390778497))
- *(config)* 将配置序列化格式从 JSON 切换为 TOML (由 @shiwuliya 提供) - ([48fc976](https://github.com/puniyu/puniyu/commit/48fc976274386311b62fa1d344807a68432b99f0))
- *(plugin)* 新增服务端插件支持 (由 @shiwuliya 提供) - ([7f15acf](https://github.com/puniyu/puniyu/commit/7f15acf148d002e33ef246b3a65a08866a44393f))
- *(protocol)* 添加账户和适配器协议定义及Bot信息结构 ([#93](https://github.com/puniyu/puniyu/pull/93)) (由 @shiwuliya 提供) (#93) - ([9611fba](https://github.com/puniyu/puniyu/commit/9611fba37182df82297bd1a4596473f9e81c2b5c))



### 🐛 Bug 修复


- *(console)* 优化控制台适配器配置与资源管理 (由 @shiwuliya 提供) - ([31184f1](https://github.com/puniyu/puniyu/commit/31184f134328fc0b193972675e2274ea53a38864))



### 🚜 重构


- *(adapter)* 重构适配器模块结构和依赖关系 (由 @shiwuliya 提供) - ([3c8e01f](https://github.com/puniyu/puniyu/commit/3c8e01f3bc7046220d06e4fe9c0a919a4f9d26f2))
- *(adapter)* 重构适配器API结构并添加模块化接口 ([#100](https://github.com/puniyu/puniyu/pull/100)) (由 @shiwuliya 提供) (#100) - ([6558f4f](https://github.com/puniyu/puniyu/commit/6558f4faed67a89a59c4c2790a7ba986581888a5))
- *(adapter)* 重构适配器信息结构与初始化逻辑 (由 @shiwuliya 提供) - ([2e45256](https://github.com/puniyu/puniyu/commit/2e4525633031ec401f058507218cb2731ac24479))
- *(adapter)* 独立account模块 (由 @shiwuliya 提供) - ([0f4c175](https://github.com/puniyu/puniyu/commit/0f4c1758ecc817c2048fec38265edbf630252cb0))
- *(config)* 添加适配器配置系统支持 (由 @shiwuliya 提供) - ([5358888](https://github.com/puniyu/puniyu/commit/5358888a950988a6be04c9b539bdac95c9b09b8d))
- *(contact)* 重构联系人类型系统，引入 trait 抽象 (由 @shiwuliya 提供) - ([c403f08](https://github.com/puniyu/puniyu/commit/c403f08b8aaebfa071995c8179c9d75a8ae532d9))
- *(core)* 重构元素宏和消息构建器实现 (由 @shiwuliya 提供) - ([abdd4ac](https://github.com/puniyu/puniyu/commit/abdd4ac369560284bc77420d25f99035d3adb4a7))
- *(core)* 重新组织模块导入顺序并清理无用导出 (由 @shiwuliya 提供) - ([8da3556](https://github.com/puniyu/puniyu/commit/8da35564bb6e99777d30d5c48497e11358c51915))
- *(element)* 重构消息元素模块结构 (由 @shiwuliya 提供) - ([2aca906](https://github.com/puniyu/puniyu/commit/2aca906d9f9d44e77753e3784539be24a16f878d))
- *(error)* 统一错误处理机制 (由 @shiwuliya 提供) - ([586272d](https://github.com/puniyu/puniyu/commit/586272ddf8828cfa7af809bc01de95d1245c4d9b))
- *(event)* 重构事件系统并重命名事件总线为事件模块 (由 @shiwuliya 提供) (#98) - ([39093d4](https://github.com/puniyu/puniyu/commit/39093d4da202aaac142cd134d38e55e9a40b526d))
- *(project)* 重构项目结构 (由 @shiwuliya 提供) - ([b1b389f](https://github.com/puniyu/puniyu/commit/b1b389f25dae5899b49133be88ba348930117972))
- *(workspace)* 重构项目结构和依赖管理 (由 @shiwuliya 提供) - ([520087e](https://github.com/puniyu/puniyu/commit/520087e38b49ff9e3aabe04be946038b81114b4d))

- 重构工作区crates ([#53](https://github.com/puniyu/puniyu/pull/53)) (由 @shiwuliya 提供) (#53) - ([f55ab51](https://github.com/puniyu/puniyu/commit/f55ab519b9275c19773f16acfddff0c44db8e48c))


### 🎨 样式


- *(code)* 项目格式化 (由 @shiwuliya 提供) - ([dc3d850](https://github.com/puniyu/puniyu/commit/dc3d850bcca149de821bc5ec700ae98f567f4a79))



### ⚙️ 杂项


- *(version)* 导出version类型 (由 @shiwuliya 提供) - ([7d8d041](https://github.com/puniyu/puniyu/commit/7d8d0410618a915a7084caa2cfb8bcb29c0754d2))
- *(workflow)* 修复发布ci (由 @shiwuliya 提供) - ([7cd84d6](https://github.com/puniyu/puniyu/commit/7cd84d6398285b00be4792942110421b71122cbe))



### Fest


- *(server)* 添加服务器路由支持 (由 @shiwuliya 提供) - ([7360323](https://github.com/puniyu/puniyu/commit/7360323b64400834013ad246d483dadf01db53ea))




### 贡献者

* @shiwuliya
* @puniyu[bot]
