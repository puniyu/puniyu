# 变更日志
## [0.7.1](https://github.com/puniyu/puniyu/compare/puniyu_types-v0.7.0...puniyu_types-v0.7.1) - 2026-01-07


### ⛰️ 新功能


- *(element)* 添加多种元素类型的访问方法 (由 @shiwuliya 提供) - ([7938469](https://github.com/puniyu/puniyu/commit/7938469d20165e36b24d287ecd68fc0324c0fad7))




### 贡献者

* @shiwuliya

## [0.7.0](https://github.com/puniyu/puniyu/compare/types-v0.6.0...types-v0.7.0) (2026-01-06)


### ✨ 新功能

* **adapter:** 实现server适配器 ([#99](https://github.com/puniyu/puniyu/issues/99)) ([e1671ad](https://github.com/puniyu/puniyu/commit/e1671ad8ebc180f066be1ec58a508194d66e4850))
* **command:** 添加命令冷却期检查功能 ([09025b7](https://github.com/puniyu/puniyu/commit/09025b72c088aa772875b75140d73d6889ec4c3a))
* **command:** 添加命令权限控制功能 ([cc0013a](https://github.com/puniyu/puniyu/commit/cc0013aff04d8efea0b9cdda3f11eae4d1eac97b))
* **config:** 添加全局命令前缀配置并优化模块结构 ([e0e5bb5](https://github.com/puniyu/puniyu/commit/e0e5bb5ac9eb24a37189a3d50b42d3db8db58dde))
* **element:** 重构消息元素类型和协议定义 ([e0d01c2](https://github.com/puniyu/puniyu/commit/e0d01c24f48d68a655cb19ed909938e4cd433a1c))
* **macro:** 重构宏系统并增强命令参数支持 ([e8f3c23](https://github.com/puniyu/puniyu/commit/e8f3c23e42cd94fb567dce569cbe4477014300b1))
* **plugin:** 支持命令参数的位置和命名模式 ([85e92d4](https://github.com/puniyu/puniyu/commit/85e92d4ec50367ad3d1e1194ee1542ce74dd82dd))
* **plugin:** 添加基础插件及状态命令功能 ([2503696](https://github.com/puniyu/puniyu/commit/2503696fa293909244f5110e8504ae00bf8b962b))
* **protocol:** 实现事件协议和字节数据类型支持 ([ec854ca](https://github.com/puniyu/puniyu/commit/ec854caf1c2ee6e722c295cc317721c87539953e))
* **protocol:** 添加联系人和发送者协议支持并重构元素处理 ([e3e6bba](https://github.com/puniyu/puniyu/commit/e3e6bbabb68d714ee01c1cc482e1055a84d88222))
* **protocol:** 添加账户和适配器协议定义及Bot信息结构 ([#93](https://github.com/puniyu/puniyu/issues/93)) ([9611fba](https://github.com/puniyu/puniyu/commit/9611fba37182df82297bd1a4596473f9e81c2b5c))
* **server:** 实现服务器控制功能并完善插件卸载功能 ([3cab133](https://github.com/puniyu/puniyu/commit/3cab133589d93d2d7592ed867db245999c774723))
* **types:** 为事件和消息类型添加序列化支持 ([bc27894](https://github.com/puniyu/puniyu/commit/bc278944120bd2ea04e676f14018deaba309bf73))
* **types:** 引入 derive_builder 以简化 AdapterInfo 构建 ([9c894fd](https://github.com/puniyu/puniyu/commit/9c894fdf06b49f7f5f73141d03f7769dfc807c5e))
* **types:** 更新图片元素结构与消息处理逻辑 ([9b69689](https://github.com/puniyu/puniyu/commit/9b69689c679b3baa2a2d8acff99661b3e22f1766))


### 🐛 错误修复

* **event:** 优化事件分发机制 ([e634a3c](https://github.com/puniyu/puniyu/commit/e634a3cf25958fe96db62d4b6f56b78aaf843bb8))
* **macro:** 修复宏中类型引用路径问题 ([a62296f](https://github.com/puniyu/puniyu/commit/a62296fd3da83a61a2d0455f8c2281760c3552cc))
* **puniyu_plugin:** 修正plugin宏获取crate name错误 ([b8fdbd7](https://github.com/puniyu/puniyu/commit/b8fdbd7b16371cdcbdee23d90c3075aff4cc4ee4))


### 🎨 代码样式

* **code:** 项目格式化 ([dc3d850](https://github.com/puniyu/puniyu/commit/dc3d850bcca149de821bc5ec700ae98f567f4a79))


### 🔧 其他更新

* **puniyu_types:** 修改图片数据结构类型 ([b9c195b](https://github.com/puniyu/puniyu/commit/b9c195b46d86b16a1688874036ce6cfed16fe308))
* **puniyu_types:** 增强消息上下文与事件处理功能 ([f44068f](https://github.com/puniyu/puniyu/commit/f44068fe4612659f23ad020e0605a562244fee50))
* **puniyu_types:** 更新 AdapterInfo 结构体字段默认值及构造宏 ([76e8ebe](https://github.com/puniyu/puniyu/commit/76e8ebe2b6d2a60ebc935378fc4a98bdc47b414b))
* release main ([63f497e](https://github.com/puniyu/puniyu/commit/63f497e6284153150f15b9c17e22bd84d532415c))


### ♻️ 代码重构

* **account:** 重构 AccountInfo 的结构体 ([0ae3ee2](https://github.com/puniyu/puniyu/commit/0ae3ee2ff7a242a6402458a124604676a191c2a8))
* **adapter:** 将时间类型从 SystemTime 替换为 OffsetDateTime ([3155084](https://github.com/puniyu/puniyu/commit/3155084efb517ec67e9ca4ce31d83fa008d6d0ea))
* **adapter:** 重构适配器API结构并添加模块化接口 ([#100](https://github.com/puniyu/puniyu/issues/100)) ([6558f4f](https://github.com/puniyu/puniyu/commit/6558f4faed67a89a59c4c2790a7ba986581888a5))
* **adapter:** 重构适配器信息结构与初始化逻辑 ([2e45256](https://github.com/puniyu/puniyu/commit/2e4525633031ec401f058507218cb2731ac24479))
* **adapter:** 重构适配器模块结构和依赖关系 ([3c8e01f](https://github.com/puniyu/puniyu/commit/3c8e01f3bc7046220d06e4fe9c0a919a4f9d26f2))
* **app:** 重构应用构建器和适配器加载逻辑 ([58ddb3f](https://github.com/puniyu/puniyu/commit/58ddb3f0decf9b50b2e1270a8da6dc914a88dfd0))
* **bus:** 重构事件总线模块 ([7f53e97](https://github.com/puniyu/puniyu/commit/7f53e9731a5f58831c758c372ef46171a8e5208b))
* **command:** 为ArgValue实现Display trait ([9db7d29](https://github.com/puniyu/puniyu/commit/9db7d2979dee942e70a3cb9feaeb967e39e6dd97))
* **command:** 修正日志格式 ([b8fdbd7](https://github.com/puniyu/puniyu/commit/b8fdbd7b16371cdcbdee23d90c3075aff4cc4ee4))
* **command:** 重构命令处理系统 ([549db91](https://github.com/puniyu/puniyu/commit/549db91d4bd063616c85bced71f3ea69431bf6c0))
* **config:** 重构配置模块并新增适配器配置支持 ([26874a2](https://github.com/puniyu/puniyu/commit/26874a22ac9114d487ac56767927b7f1b8bbe205))
* **context:** 重构消息上下文中的艾特和元素处理逻辑 ([70ba0a5](https://github.com/puniyu/puniyu/commit/70ba0a544552c82d6551eed5409485cb02ec26d7))
* **core:** 重新组织模块导入顺序并清理无用导出 ([8da3556](https://github.com/puniyu/puniyu/commit/8da35564bb6e99777d30d5c48497e11358c51915))
* **core:** 重构元素宏和消息构建器实现 ([abdd4ac](https://github.com/puniyu/puniyu/commit/abdd4ac369560284bc77420d25f99035d3adb4a7))
* **core:** 重构命令匹配器与上下文处理逻辑 ([3aca600](https://github.com/puniyu/puniyu/commit/3aca600a94a09079ecbd8e84cf51376fc1222a99))
* **element:** 修改图片文件字段类型从Vec&lt;u8&gt;到String ([744e7aa](https://github.com/puniyu/puniyu/commit/744e7aa6b8daaf43a7720ad7b65156ef63ef572e))
* **element:** 重构消息元素模块结构 ([2aca906](https://github.com/puniyu/puniyu/commit/2aca906d9f9d44e77753e3784539be24a16f878d))
* **event:** 重构事件系统并重命名事件总线为事件模块 ([39093d4](https://github.com/puniyu/puniyu/commit/39093d4da202aaac142cd134d38e55e9a40b526d))
* **puniyu_common:** 添加 stable 特性并重构版本信息模块 ([5509982](https://github.com/puniyu/puniyu/commit/550998233d7ee02150b15c16f7ef3fa139086137))
* **puniyu_core:** 移除重复的 version 模块引用 ([5509982](https://github.com/puniyu/puniyu/commit/550998233d7ee02150b15c16f7ef3fa139086137))
* **puniyu_server:** 优化 API 路由结构与响应类型泛型化 ([5509982](https://github.com/puniyu/puniyu/commit/550998233d7ee02150b15c16f7ef3fa139086137))
* **server:** 将服务器命令通道从 tokio mpsc 迁移至 flume ([1345cae](https://github.com/puniyu/puniyu/commit/1345caefb4b07f094d00521fd9286eb6b33b9ab5))
* **types:** 重构构造器实现 ([d0684d2](https://github.com/puniyu/puniyu/commit/d0684d273ca1efc67608daae50df71eecd888ded))
* **types:** 重构构造器实现 ([#92](https://github.com/puniyu/puniyu/issues/92)) ([db1763a](https://github.com/puniyu/puniyu/commit/db1763abd59f270a7c78f8c2b0025f05b40954c4))
* **types:** 重构类型定义和宏实现 ([23561c9](https://github.com/puniyu/puniyu/commit/23561c9d33724d59b9b22228f4d2b192efad8faf))
* **version:** 为Version结构体实现FromStr trait ([2c401d3](https://github.com/puniyu/puniyu/commit/2c401d37d5a900cf94b9256d234b8ebde1f10949))


### ✅ 测试相关

* **adapter:** 添加适配器类型枚举的单元测试 ([31fd2b3](https://github.com/puniyu/puniyu/commit/31fd2b3b8abd6af4b633d620046f05c3385838e4))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_config bumped from 0.6.0 to 0.7.0

## [0.6.0](https://github.com/puniyu/puniyu/compare/types-v0.5.12...types-v0.6.0) (2025-12-02)


### ✨ 新功能

* **config:** 添加全局命令前缀配置并优化模块结构 ([e0e5bb5](https://github.com/puniyu/puniyu/commit/e0e5bb5ac9eb24a37189a3d50b42d3db8db58dde))
* **macro:** 重构宏系统并增强命令参数支持 ([e8f3c23](https://github.com/puniyu/puniyu/commit/e8f3c23e42cd94fb567dce569cbe4477014300b1))
* **plugin:** 支持命令参数的位置和命名模式 ([85e92d4](https://github.com/puniyu/puniyu/commit/85e92d4ec50367ad3d1e1194ee1542ce74dd82dd))
* **server:** 实现服务器控制功能并完善插件卸载功能 ([3cab133](https://github.com/puniyu/puniyu/commit/3cab133589d93d2d7592ed867db245999c774723))
* **types:** 更新图片元素结构与消息处理逻辑 ([9b69689](https://github.com/puniyu/puniyu/commit/9b69689c679b3baa2a2d8acff99661b3e22f1766))


### 🐛 错误修复

* **puniyu_plugin:** 修正plugin宏获取crate name错误 ([b8fdbd7](https://github.com/puniyu/puniyu/commit/b8fdbd7b16371cdcbdee23d90c3075aff4cc4ee4))


### 🎨 代码样式

* **mes:** 优化消息发送日志记录格式 ([0a8336a](https://github.com/puniyu/puniyu/commit/0a8336a777a568c13d27c7e84a5952c40c0d3055))


### 🔧 其他更新

* **puniyu_types:** 增强消息上下文与事件处理功能 ([f44068f](https://github.com/puniyu/puniyu/commit/f44068fe4612659f23ad020e0605a562244fee50))


### ♻️ 代码重构

* **command:** 修正日志格式 ([b8fdbd7](https://github.com/puniyu/puniyu/commit/b8fdbd7b16371cdcbdee23d90c3075aff4cc4ee4))
* **config:** 重构配置模块并新增适配器配置支持 ([26874a2](https://github.com/puniyu/puniyu/commit/26874a22ac9114d487ac56767927b7f1b8bbe205))
* **context:** 重构消息上下文中的艾特和元素处理逻辑 ([70ba0a5](https://github.com/puniyu/puniyu/commit/70ba0a544552c82d6551eed5409485cb02ec26d7))
* **core:** 重新组织模块导入顺序并清理无用导出 ([8da3556](https://github.com/puniyu/puniyu/commit/8da35564bb6e99777d30d5c48497e11358c51915))
* **element:** 重构消息元素模块结构 ([2aca906](https://github.com/puniyu/puniyu/commit/2aca906d9f9d44e77753e3784539be24a16f878d))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_config bumped from 0.5.12 to 0.6.0

## [0.5.12](https://github.com/puniyu/puniyu/compare/types-v0.5.11...types-v0.5.12) (2025-11-24)


### 🔧 其他更新

* **types:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_config bumped from 0.5.11 to 0.5.12

## [0.5.11](https://github.com/puniyu/puniyu/compare/types-v0.5.10...types-v0.5.11) (2025-11-23)


### 🔧 其他更新

* **types:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_config bumped from 0.5.10 to 0.5.11

## [0.5.10](https://github.com/puniyu/puniyu/compare/types-v0.5.9...types-v0.5.10) (2025-11-23)


### ♻️ 代码重构

* **command:** 重构命令处理结果类型和参数验证 ([58d4eeb](https://github.com/puniyu/puniyu/commit/58d4eebb41cacabc7663b40a93181b789feb1e0a))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_config bumped from 0.5.9 to 0.5.10

## [0.5.9](https://github.com/puniyu/puniyu/compare/types-v0.5.8...types-v0.5.9) (2025-11-23)


### 🔧 其他更新

* **types:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_config bumped from 0.5.8 to 0.5.9

## [0.5.8](https://github.com/puniyu/puniyu/compare/types-v0.5.7...types-v0.5.8) (2025-11-23)


### 🔧 其他更新

* **types:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_config bumped from 0.5.7 to 0.5.8

## [0.5.7](https://github.com/puniyu/puniyu/compare/types-v0.5.6...types-v0.5.7) (2025-11-23)


### 🔧 其他更新

* **types:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_config bumped from 0.5.6 to 0.5.7

## [0.5.6](https://github.com/puniyu/puniyu/compare/types-v0.5.4...types-v0.5.6) (2025-11-23)


### 🔧 其他更新

* **types:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_config bumped from 0.5.4 to 0.5.6

## [0.5.4](https://github.com/puniyu/puniyu/compare/types-v0.5.1...types-v0.5.4) (2025-11-23)


### 🔧 其他更新

* **types:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_config bumped from 0.5.1 to 0.5.4

## [0.5.1](https://github.com/puniyu/puniyu/compare/types-v0.5.0...types-v0.5.1) (2025-11-23)


### 🔧 其他更新

* **puniyu_types:** 添加包描述信息 ([ec9bf4d](https://github.com/puniyu/puniyu/commit/ec9bf4de5ffe6f2747437e0e9f387946713ce1bc))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_config bumped from 0.5.0 to 0.5.1

## [0.5.0](https://github.com/puniyu/puniyu/compare/types-v0.4.1...types-v0.5.0) (2025-11-23)


### ✨ 新功能

* **config:** 将配置序列化格式从 JSON 切换为 TOML ([48fc976](https://github.com/puniyu/puniyu/commit/48fc976274386311b62fa1d344807a68432b99f0))
* **plugin:** 添加插件配置支持 ([dc7d1eb](https://github.com/puniyu/puniyu/commit/dc7d1ebcf2245f53f3a58b203edd405aa7cc8c1c))


### 🔧 其他更新

* **deps:** 更新 tokio 依赖并启用完整特性 ([1b717fa](https://github.com/puniyu/puniyu/commit/1b717faacc17a23de1ec0a7856744c80d97b1a85))


### ♻️ 代码重构

* **config:** 添加适配器配置系统支持 ([5358888](https://github.com/puniyu/puniyu/commit/5358888a950988a6be04c9b539bdac95c9b09b8d))
* 重构工作区crates ([#53](https://github.com/puniyu/puniyu/issues/53)) ([f55ab51](https://github.com/puniyu/puniyu/commit/f55ab519b9275c19773f16acfddff0c44db8e48c))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_config bumped from 0.4.1 to 0.5.0
