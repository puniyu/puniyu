# 变更日志

## [0.7.9](https://github.com/puniyu/puniyu/compare/core-v0.7.8...core-v0.7.9)

### ⛰️ 新功能


- 添加命令解析器和配置管理功能 - ([075d485](https://github.com/puniyu/puniyu/commit/075d48589a9da7ec45fbd88c60cbe5a3435a3163))

### 🚜 重构


- *(core)* 重构部分核心实现 ([#150](https://github.com/puniyu/puniyu/pull/150)) - ([e06459e](https://github.com/puniyu/puniyu/commit/e06459e40f1ec37f8e0de7427e6905c9bb295d5e))
- 重构core, 抽象层实现 - ([62a2862](https://github.com/puniyu/puniyu/commit/62a28625debc8bd3407beb464ae9e550bec9ea9e))

## [0.7.7](https://github.com/puniyu/puniyu/compare/core-v0.7.6...core-v0.7.7) - 2026-01-11


### ⚙️ 杂项



- Updated the following local packages: puniyu_types, puniyu_registry, puniyu_event, puniyu_server - ([0000000](https://github.com/puniyu/puniyu/commit/0000000))

## [0.7.6](https://github.com/puniyu/puniyu/compare/core-v0.7.5...core-v0.7.6) - 2026-01-11


### ⛰️ 新功能


- *(adapter)* 实现适配器服务器的消息转发功能 (由 @wuliya336 提供) - ([e240974](https://github.com/puniyu/puniyu/commit/e2409749a42e40eeff725bb98d3c5d987d324320))

- Hook功能实现 ([#112](https://github.com/puniyu/puniyu/pull/112)) (由 @wuliya336 提供) (#112) - ([1e7bbf7](https://github.com/puniyu/puniyu/commit/1e7bbf7b6d25fcfb5c8fcedc6a68f29ff6b8c12e))


### 🚜 重构


- *(macros)* 重构proc-macro宏 ([#119](https://github.com/puniyu/puniyu/pull/119)) (由 @wuliya336 提供) (#119) - ([9942a1b](https://github.com/puniyu/puniyu/commit/9942a1baa348377af978d9224510316a0595a5f3))




### 贡献者

* @puniyu[bot]
* @wuliya336
## [0.7.5](https://github.com/puniyu/puniyu/compare/core-v0.7.4...core-v0.7.5) - 2026-01-11


### ⛰️ 新功能


- *(adapter)* 实现适配器服务器的消息转发功能 (由 @wuliya336 提供) - ([e240974](https://github.com/puniyu/puniyu/commit/e2409749a42e40eeff725bb98d3c5d987d324320))

- Hook功能实现 ([#112](https://github.com/puniyu/puniyu/pull/112)) (由 @wuliya336 提供) (#112) - ([1e7bbf7](https://github.com/puniyu/puniyu/commit/1e7bbf7b6d25fcfb5c8fcedc6a68f29ff6b8c12e))


### 🚜 重构


- *(macros)* 重构proc-macro宏 ([#119](https://github.com/puniyu/puniyu/pull/119)) (由 @wuliya336 提供) (#119) - ([9942a1b](https://github.com/puniyu/puniyu/commit/9942a1baa348377af978d9224510316a0595a5f3))




### 贡献者

* @wuliya336
## [0.7.4](https://github.com/puniyu/puniyu/compare/core-v0.7.3...core-v0.7.4) - 2026-01-07


### ⛰️ 新功能


- *(adapter)* 支持配置文件读取功能 (由 @shiwuliya 提供) - ([a9fc6e2](https://github.com/puniyu/puniyu/commit/a9fc6e2aed53370db0c78a0035c37eec53114445))
- *(cli)* 引入命令行参数解析功能 (由 @shiwuliya 提供) - ([11e3137](https://github.com/puniyu/puniyu/commit/11e31372aca53c35f15e8cab8b3067af353d25a7))
- *(config)* 重构配置管理模块以支持动态注册与热重载 (由 @shiwuliya 提供) - ([f3234c1](https://github.com/puniyu/puniyu/commit/f3234c16ea7d49b4cae2cdd0bda024f390778497))
- *(core)* 引入系统信息模块并优化运行时间获取逻辑 (由 @shiwuliya 提供) - ([f0dd848](https://github.com/puniyu/puniyu/commit/f0dd8488d25708e9d01485d28193f1b17cb94dba))
- *(core)* 将 puniyu_logger 设置为可选依赖并更新版本获取逻辑 (由 @shiwuliya 提供) - ([f49677a](https://github.com/puniyu/puniyu/commit/f49677ad1bcf2c5b64ff699241c427a7a7300cc0))
- *(protocol)* 添加账户和适配器协议定义及Bot信息结构 ([#93](https://github.com/puniyu/puniyu/pull/93)) (由 @shiwuliya 提供) (#93) - ([9611fba](https://github.com/puniyu/puniyu/commit/9611fba37182df82297bd1a4596473f9e81c2b5c))
- *(protocol)* 实现事件协议和字节数据类型支持 (由 @shiwuliya 提供) (#91) - ([ec854ca](https://github.com/puniyu/puniyu/commit/ec854caf1c2ee6e722c295cc317721c87539953e))
- *(protocol)* 添加联系人和发送者协议支持并重构元素处理 (由 @shiwuliya 提供) (#91) - ([e3e6bba](https://github.com/puniyu/puniyu/commit/e3e6bbabb68d714ee01c1cc482e1055a84d88222))
- *(server)* 实现服务器控制功能并完善插件卸载功能 (由 @shiwuliya 提供) - ([3cab133](https://github.com/puniyu/puniyu/commit/3cab133589d93d2d7592ed867db245999c774723))
- *(types)* 引入 derive_builder 以简化 AdapterInfo 构建 (由 @shiwuliya 提供) - ([9c894fd](https://github.com/puniyu/puniyu/commit/9c894fdf06b49f7f5f73141d03f7769dfc807c5e))



### 🐛 Bug 修复


- *(path)* 重构工作目录设置逻辑 (由 @shiwuliya 提供) - ([df51eac](https://github.com/puniyu/puniyu/commit/df51eac9e4fe92b11df2867ebdeca78fe62b2022))

- 初始化时间计算错误 (由 @shiwuliya 提供) - ([2c53605](https://github.com/puniyu/puniyu/commit/2c536057bc8c59649863117a0e542649faad3adc))


### 🚜 重构


- *(adapter)* 优化 Avatar 类型实现并添加服务器 logo 接口 (由 @shiwuliya 提供) - ([952c18b](https://github.com/puniyu/puniyu/commit/952c18b3008a5e31fd00127dc6d2fb55568c9796))
- *(app)* 重构应用构建器和适配器加载逻辑 (由 @shiwuliya 提供) - ([58ddb3f](https://github.com/puniyu/puniyu/commit/58ddb3f0decf9b50b2e1270a8da6dc914a88dfd0))
- *(bus)* 重构事件总线模块 (由 @shiwuliya 提供) - ([7f53e97](https://github.com/puniyu/puniyu/commit/7f53e9731a5f58831c758c372ef46171a8e5208b))
- *(config)* 添加适配器配置系统支持 (由 @shiwuliya 提供) - ([5358888](https://github.com/puniyu/puniyu/commit/5358888a950988a6be04c9b539bdac95c9b09b8d))
- *(config)* 重构配置文件监听器实现 (由 @shiwuliya 提供) - ([4c99137](https://github.com/puniyu/puniyu/commit/4c9913784f5a40bcb8d13494121489ea86ce17c4))
- *(core)* 重构元素宏和消息构建器实现 (由 @shiwuliya 提供) - ([abdd4ac](https://github.com/puniyu/puniyu/commit/abdd4ac369560284bc77420d25f99035d3adb4a7))
- *(core)* 重新组织模块导入顺序并清理无用导出 (由 @shiwuliya 提供) - ([8da3556](https://github.com/puniyu/puniyu/commit/8da35564bb6e99777d30d5c48497e11358c51915))
- *(core)* 移动logo资源到core包并优化构建配置 (由 @shiwuliya 提供) - ([4e2d787](https://github.com/puniyu/puniyu/commit/4e2d787dd92318b45d1128d11ffc26c2451729c3))
- *(core)* 调整Bot模块导出方式 (由 @shiwuliya 提供) - ([d94d88c](https://github.com/puniyu/puniyu/commit/d94d88cf891d9e222fb996c54895fa062bc17fe7))
- *(event)* 重构事件系统并重命名事件总线为事件模块 (由 @shiwuliya 提供) (#98) - ([39093d4](https://github.com/puniyu/puniyu/commit/39093d4da202aaac142cd134d38e55e9a40b526d))
- *(project)* 重构项目结构 (由 @shiwuliya 提供) - ([b1b389f](https://github.com/puniyu/puniyu/commit/b1b389f25dae5899b49133be88ba348930117972))
- *(puniyu_common)* 添加 stable 特性并重构版本信息模块 (由 @shiwuliya 提供) - ([5509982](https://github.com/puniyu/puniyu/commit/550998233d7ee02150b15c16f7ef3fa139086137))

- 重构工作区crates ([#53](https://github.com/puniyu/puniyu/pull/53)) (由 @shiwuliya 提供) (#53) - ([f55ab51](https://github.com/puniyu/puniyu/commit/f55ab519b9275c19773f16acfddff0c44db8e48c))


### 🎨 样式


- *(code)* 项目格式化 (由 @shiwuliya 提供) - ([dc3d850](https://github.com/puniyu/puniyu/commit/dc3d850bcca149de821bc5ec700ae98f567f4a79))



### ⚙️ 杂项


- *(core)* 支持设置app_logo (由 @shiwuliya 提供) - ([3f4f713](https://github.com/puniyu/puniyu/commit/3f4f71344917f671468edfef639ec201440a1251))
- *(core)* 导出 puniyu_common 的 path 模块 (由 @shiwuliya 提供) - ([a95c43e](https://github.com/puniyu/puniyu/commit/a95c43e1104b4bb4aadaeaddc3d07f43dc083968))
- *(puniyu_core)* 更新 Cargo.toml 配置 (由 @shiwuliya 提供) - ([2c77c7a](https://github.com/puniyu/puniyu/commit/2c77c7a4b6dfc08e23835ae3ba29227c0a8ea43f))
- *(puniyu_logger)* 更新 puniyu_logger 依赖版本 (由 @shiwuliya 提供) - ([f949ad4](https://github.com/puniyu/puniyu/commit/f949ad40d00b11b9caa3f78fecb35fc37055742f))
- *(puniyu_types)* 更新 AdapterInfo 结构体字段默认值及构造宏 (由 @shiwuliya 提供) (#91) - ([76e8ebe](https://github.com/puniyu/puniyu/commit/76e8ebe2b6d2a60ebc935378fc4a98bdc47b414b))
- *(workflow)* 修复发布ci (由 @shiwuliya 提供) - ([7cd84d6](https://github.com/puniyu/puniyu/commit/7cd84d6398285b00be4792942110421b71122cbe))
- *(workflow)* 引入稳定版构建配置 (由 @shiwuliya 提供) - ([fd81025](https://github.com/puniyu/puniyu/commit/fd81025eee0ef359320d1a8c482004772491ce0f))



### Refcator



- 优化bot实例的使用 (由 @shiwuliya 提供) (#72) - ([73f284e](https://github.com/puniyu/puniyu/commit/73f284e8c594139d2a190fc09cb7ba460ceb4ef8))



### 贡献者

* @shiwuliya
* @puniyu[bot]
