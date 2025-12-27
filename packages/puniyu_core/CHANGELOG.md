# 变更日志

## [0.7.0](https://github.com/puniyu/puniyu/compare/core-v0.6.0...core-v0.7.0) (2025-12-27)


### ✨ 新功能

* **core:** 引入系统信息模块并优化运行时间获取逻辑 ([f0dd848](https://github.com/puniyu/puniyu/commit/f0dd8488d25708e9d01485d28193f1b17cb94dba))
* **protocol:** 实现事件协议和字节数据类型支持 ([ec854ca](https://github.com/puniyu/puniyu/commit/ec854caf1c2ee6e722c295cc317721c87539953e))
* **protocol:** 添加联系人和发送者协议支持并重构元素处理 ([e3e6bba](https://github.com/puniyu/puniyu/commit/e3e6bbabb68d714ee01c1cc482e1055a84d88222))
* **protocol:** 添加账户和适配器协议定义及Bot信息结构 ([#93](https://github.com/puniyu/puniyu/issues/93)) ([9611fba](https://github.com/puniyu/puniyu/commit/9611fba37182df82297bd1a4596473f9e81c2b5c))
* **types:** 引入 derive_builder 以简化 AdapterInfo 构建 ([9c894fd](https://github.com/puniyu/puniyu/commit/9c894fdf06b49f7f5f73141d03f7769dfc807c5e))


### 🎨 代码样式

* **code:** 项目格式化 ([dc3d850](https://github.com/puniyu/puniyu/commit/dc3d850bcca149de821bc5ec700ae98f567f4a79))


### 🔧 其他更新

* **puniyu_types:** 更新 AdapterInfo 结构体字段默认值及构造宏 ([76e8ebe](https://github.com/puniyu/puniyu/commit/76e8ebe2b6d2a60ebc935378fc4a98bdc47b414b))


### ♻️ 代码重构

* **bus:** 重构事件总线模块 ([7f53e97](https://github.com/puniyu/puniyu/commit/7f53e9731a5f58831c758c372ef46171a8e5208b))
* **core:** 重构元素宏和消息构建器实现 ([abdd4ac](https://github.com/puniyu/puniyu/commit/abdd4ac369560284bc77420d25f99035d3adb4a7))
* **puniyu_common:** 添加 stable 特性并重构版本信息模块 ([5509982](https://github.com/puniyu/puniyu/commit/550998233d7ee02150b15c16f7ef3fa139086137))
* **puniyu_core:** 移除重复的 version 模块引用 ([5509982](https://github.com/puniyu/puniyu/commit/550998233d7ee02150b15c16f7ef3fa139086137))
* **puniyu_server:** 优化 API 路由结构与响应类型泛型化 ([5509982](https://github.com/puniyu/puniyu/commit/550998233d7ee02150b15c16f7ef3fa139086137))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_server bumped from 0.6.0 to 0.7.0
    * puniyu_config bumped from 0.6.0 to 0.7.0
    * puniyu_bus bumped from 0.6.0 to 0.7.0
    * puniyu_common bumped from 0.6.0 to 0.7.0
    * puniyu_types bumped from 0.6.0 to 0.7.0
    * puniyu_registry bumped from 0.6.0 to 0.7.0

## [0.6.0](https://github.com/puniyu/puniyu/compare/core-v0.5.12...core-v0.6.0) (2025-12-02)


### ✨ 新功能

* **cli:** 引入命令行参数解析功能 ([11e3137](https://github.com/puniyu/puniyu/commit/11e31372aca53c35f15e8cab8b3067af353d25a7))
* **server:** 实现服务器控制功能并完善插件卸载功能 ([3cab133](https://github.com/puniyu/puniyu/commit/3cab133589d93d2d7592ed867db245999c774723))


### 🔧 其他更新

* **puniyu_core:** 更新 Cargo.toml 配置 ([2c77c7a](https://github.com/puniyu/puniyu/commit/2c77c7a4b6dfc08e23835ae3ba29227c0a8ea43f))
* **puniyu_logger:** 更新 puniyu_logger 依赖版本 ([f949ad4](https://github.com/puniyu/puniyu/commit/f949ad40d00b11b9caa3f78fecb35fc37055742f))


### ♻️ 代码重构

* **core:** 重新组织模块导入顺序并清理无用导出 ([8da3556](https://github.com/puniyu/puniyu/commit/8da35564bb6e99777d30d5c48497e11358c51915))


### 🎡 持续集成

* **workflow:** 引入稳定版构建配置 ([fd81025](https://github.com/puniyu/puniyu/commit/fd81025eee0ef359320d1a8c482004772491ce0f))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_server bumped from 0.5.12 to 0.6.0
    * puniyu_config bumped from 0.5.12 to 0.6.0
    * puniyu_bus bumped from 0.5.12 to 0.6.0
    * puniyu_common bumped from 0.5.12 to 0.6.0
    * puniyu_types bumped from 0.5.12 to 0.6.0
    * puniyu_registry bumped from 0.5.12 to 0.6.0

## [0.5.12](https://github.com/puniyu/puniyu/compare/core-v0.5.11...core-v0.5.12) (2025-11-24)


### ♻️ 代码重构

* **core:** 移动logo资源到core包并优化构建配置 ([4e2d787](https://github.com/puniyu/puniyu/commit/4e2d787dd92318b45d1128d11ffc26c2451729c3))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_server bumped from 0.5.11 to 0.5.12
    * puniyu_config bumped from 0.5.11 to 0.5.12
    * puniyu_bus bumped from 0.5.11 to 0.5.12
    * puniyu_common bumped from 0.5.11 to 0.5.12
    * puniyu_types bumped from 0.5.11 to 0.5.12
    * puniyu_registry bumped from 0.5.11 to 0.5.12

## [0.5.11](https://github.com/puniyu/puniyu/compare/core-v0.5.10...core-v0.5.11) (2025-11-23)


### 🔧 其他更新

* **core:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_server bumped from 0.5.10 to 0.5.11
    * puniyu_config bumped from 0.5.10 to 0.5.11
    * puniyu_bus bumped from 0.5.10 to 0.5.11
    * puniyu_common bumped from 0.5.10 to 0.5.11
    * puniyu_types bumped from 0.5.10 to 0.5.11
    * puniyu_registry bumped from 0.5.10 to 0.5.11

## [0.5.10](https://github.com/puniyu/puniyu/compare/core-v0.5.9...core-v0.5.10) (2025-11-23)


### 🔧 其他更新

* **core:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_server bumped from 0.5.9 to 0.5.10
    * puniyu_config bumped from 0.5.9 to 0.5.10
    * puniyu_bus bumped from 0.5.9 to 0.5.10
    * puniyu_common bumped from 0.5.9 to 0.5.10
    * puniyu_types bumped from 0.5.9 to 0.5.10
    * puniyu_registry bumped from 0.5.9 to 0.5.10

## [0.5.9](https://github.com/puniyu/puniyu/compare/core-v0.5.8...core-v0.5.9) (2025-11-23)


### 🔧 其他更新

* **core:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_server bumped from 0.5.8 to 0.5.9
    * puniyu_config bumped from 0.5.8 to 0.5.9
    * puniyu_bus bumped from 0.5.8 to 0.5.9
    * puniyu_common bumped from 0.5.8 to 0.5.9
    * puniyu_types bumped from 0.5.8 to 0.5.9
    * puniyu_registry bumped from 0.5.8 to 0.5.9

## [0.5.8](https://github.com/puniyu/puniyu/compare/core-v0.5.7...core-v0.5.8) (2025-11-23)


### 🔧 其他更新

* **core:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_server bumped from 0.5.7 to 0.5.8
    * puniyu_config bumped from 0.5.7 to 0.5.8
    * puniyu_bus bumped from 0.4.10 to 0.5.8
    * puniyu_common bumped from 0.5.7 to 0.5.8
    * puniyu_types bumped from 0.5.7 to 0.5.8
    * puniyu_registry bumped from 0.5.7 to 0.5.8

## [0.5.7](https://github.com/puniyu/puniyu/compare/core-v0.5.6...core-v0.5.7) (2025-11-23)


### 🔧 其他更新

* release main ([45cd505](https://github.com/puniyu/puniyu/commit/45cd5054d26007f5a499cf3a0329f1e0435b1deb))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_server bumped from 0.5.6 to 0.5.7
    * puniyu_config bumped from 0.5.6 to 0.5.7
    * puniyu_bus bumped from 0.4.9 to 0.4.10
    * puniyu_common bumped from 0.5.6 to 0.5.7
    * puniyu_types bumped from 0.5.6 to 0.5.7
    * puniyu_registry bumped from 0.5.6 to 0.5.7

## [0.5.6](https://github.com/puniyu/puniyu/compare/core-v0.5.6...core-v0.5.6) (2025-11-23)


### 🔧 其他更新

* **core:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_config bumped from 0.5.4 to 0.5.6
    * puniyu_bus bumped from 0.4.8 to 0.4.9
    * puniyu_common bumped from 0.5.4 to 0.5.6
    * puniyu_types bumped from 0.5.4 to 0.5.6
    * puniyu_registry bumped from 0.5.5 to 0.5.6

## [0.5.6](https://github.com/puniyu/puniyu/compare/core-v0.5.5...core-v0.5.6) (2025-11-23)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_server bumped from 0.5.5 to 0.5.6
    * puniyu_bus bumped from 0.4.7 to 0.4.8
    * puniyu_registry bumped from 0.5.4 to 0.5.5

## [0.5.5](https://github.com/puniyu/puniyu/compare/core-v0.5.4...core-v0.5.5) (2025-11-23)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_server bumped from 0.5.4 to 0.5.5
    * puniyu_bus bumped from 0.4.6 to 0.4.7
    * puniyu_registry bumped from 0.5.3 to 0.5.4

## [0.5.4](https://github.com/puniyu/puniyu/compare/core-v0.5.3...core-v0.5.4) (2025-11-23)


### 🔧 其他更新

* **core:** 支持设置app_logo ([3f4f713](https://github.com/puniyu/puniyu/commit/3f4f71344917f671468edfef639ec201440a1251))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_server bumped from 0.5.2 to 0.5.4
    * puniyu_config bumped from 0.5.1 to 0.5.4
    * puniyu_bus bumped from 0.4.5 to 0.4.6
    * puniyu_common bumped from 0.5.1 to 0.5.4
    * puniyu_types bumped from 0.5.1 to 0.5.4
    * puniyu_registry bumped from 0.5.2 to 0.5.3

## [0.5.3](https://github.com/puniyu/puniyu/compare/core-v0.5.2...core-v0.5.3) (2025-11-23)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_server bumped from 0.5.1 to 0.5.2
    * puniyu_bus bumped from 0.4.4 to 0.4.5
    * puniyu_registry bumped from 0.5.1 to 0.5.2

## [0.5.2](https://github.com/puniyu/puniyu/compare/core-v0.5.1...core-v0.5.2) (2025-11-23)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_bus bumped from 0.4.3 to 0.4.4

## [0.5.1](https://github.com/puniyu/puniyu/compare/core-v0.5.0...core-v0.5.1) (2025-11-23)


### 🔧 其他更新

* **core:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_server bumped from 0.5.0 to 0.5.1
    * puniyu_config bumped from 0.5.0 to 0.5.1
    * puniyu_bus bumped from 0.4.2 to 0.4.3
    * puniyu_common bumped from 0.5.0 to 0.5.1
    * puniyu_types bumped from 0.5.0 to 0.5.1
    * puniyu_registry bumped from 0.5.0 to 0.5.1

## [0.5.0](https://github.com/puniyu/puniyu/compare/core-v0.4.1...core-v0.5.0) (2025-11-23)


### ✨ 新功能

* **adapter:** 支持配置文件读取功能 ([a9fc6e2](https://github.com/puniyu/puniyu/commit/a9fc6e2aed53370db0c78a0035c37eec53114445))
* **config:** 重构配置管理模块以支持动态注册与热重载 ([f3234c1](https://github.com/puniyu/puniyu/commit/f3234c16ea7d49b4cae2cdd0bda024f390778497))


### 🔧 其他更新

* **deps:** 更新依赖版本 ([63551d7](https://github.com/puniyu/puniyu/commit/63551d722a048e4e5e0479fe421d7190a8b7c7e9))


### ♻️ 代码重构

* **adapter:** 优化 Avatar 类型实现并添加服务器 logo 接口 ([952c18b](https://github.com/puniyu/puniyu/commit/952c18b3008a5e31fd00127dc6d2fb55568c9796))
* **config:** 添加适配器配置系统支持 ([5358888](https://github.com/puniyu/puniyu/commit/5358888a950988a6be04c9b539bdac95c9b09b8d))
* **core:** 调整Bot模块导出方式 ([d94d88c](https://github.com/puniyu/puniyu/commit/d94d88cf891d9e222fb996c54895fa062bc17fe7))
* 重构工作区crates ([#53](https://github.com/puniyu/puniyu/issues/53)) ([f55ab51](https://github.com/puniyu/puniyu/commit/f55ab519b9275c19773f16acfddff0c44db8e48c))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_server bumped from 0.4.1 to 0.5.0
    * puniyu_config bumped from 0.4.1 to 0.5.0
    * puniyu_bus bumped from 0.4.1 to 0.4.2
    * puniyu_common bumped from 0.4.1 to 0.5.0
    * puniyu_types bumped from 0.4.1 to 0.5.0
    * puniyu_registry bumped from 0.4.1 to 0.5.0

## [0.4.1](https://github.com/puniyu/puniyu/compare/core-v0.4.0...core-v0.4.1) (2025-11-16)


### 🐛 错误修复

* **path:** 重构工作目录设置逻辑 ([df51eac](https://github.com/puniyu/puniyu/commit/df51eac9e4fe92b11df2867ebdeca78fe62b2022))


### 🔧 其他更新

* **core:** 导出 puniyu_common 的 path 模块 ([a95c43e](https://github.com/puniyu/puniyu/commit/a95c43e1104b4bb4aadaeaddc3d07f43dc083968))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_config bumped from 0.4.0 to 0.4.1
    * puniyu_bot bumped from 0.4.0 to 0.4.1
    * puniyu_task bumped from 0.4.0 to 0.4.1
    * puniyu_server bumped from 0.4.0 to 0.4.1
    * puniyu_builder bumped from 0.4.0 to 0.4.1
    * puniyu_registry bumped from 0.4.0 to 0.4.1
    * puniyu_common bumped from 0.4.0 to 0.4.1
    * puniyu_event_bus bumped from 0.4.0 to 0.4.1

## [0.4.0](https://github.com/puniyu/puniyu/compare/core-v0.3.1...core-v0.4.0) (2025-11-16)


### 🔧 其他更新

* **core:** Synchronize puniyu versions


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_config bumped from 0.3.0 to 0.4.0
    * puniyu_bot bumped from 0.3.0 to 0.4.0
    * puniyu_task bumped from 0.1.10 to 0.4.0
    * puniyu_server bumped from 0.2.0 to 0.4.0
    * puniyu_builder bumped from 0.3.0 to 0.4.0
    * puniyu_registry bumped from 0.3.0 to 0.4.0
    * puniyu_common bumped from 0.3.0 to 0.4.0
    * puniyu_event_bus bumped from 0.3.0 to 0.4.0

## [0.3.1](https://github.com/puniyu/puniyu/compare/core-v0.3.0...core-v0.3.1) (2025-11-15)


### 🔧 其他更新

* **release:** 调整发布配置并更新依赖版本 ([381ca55](https://github.com/puniyu/puniyu/commit/381ca558c7ca6b4bcc9b9386ecc228e8679f7305))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_config bumped from 0.2.0 to 0.3.0
    * puniyu_bot bumped from 0.2.0 to 0.3.0
    * puniyu_task bumped from 0.1.9 to 0.1.10
    * puniyu_builder bumped from 0.2.0 to 0.3.0
    * puniyu_registry bumped from 0.2.0 to 0.3.0
    * puniyu_common bumped from 0.2.0 to 0.3.0
    * puniyu_event_bus bumped from 0.2.0 to 0.3.0

## [0.3.0](https://github.com/puniyu/puniyu/compare/core-v0.2.0...core-v0.3.0) (2025-11-15)


### ✨ 新功能

* **core:** 将 puniyu_logger 设置为可选依赖并更新版本获取逻辑 ([f49677a](https://github.com/puniyu/puniyu/commit/f49677ad1bcf2c5b64ff699241c427a7a7300cc0))


### ♻️ 代码重构

* **config:** 重构配置文件监听器实现 ([4c99137](https://github.com/puniyu/puniyu/commit/4c9913784f5a40bcb8d13494121489ea86ce17c4))
* **project:** 重构项目结构 ([b1b389f](https://github.com/puniyu/puniyu/commit/b1b389f25dae5899b49133be88ba348930117972))
