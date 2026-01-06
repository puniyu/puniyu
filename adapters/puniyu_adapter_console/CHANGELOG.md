# 变更日志

## [0.4.0](https://github.com/puniyu/puniyu/compare/adapter-console-v0.3.0...adapter-console-v0.4.0) (2026-01-06)


### ✨ 新功能

* **adapter:** 实现server适配器 ([#99](https://github.com/puniyu/puniyu/issues/99)) ([e1671ad](https://github.com/puniyu/puniyu/commit/e1671ad8ebc180f066be1ec58a508194d66e4850))
* **element:** 重构消息元素类型和协议定义 ([e0d01c2](https://github.com/puniyu/puniyu/commit/e0d01c24f48d68a655cb19ed909938e4cd433a1c))
* **protocol:** 实现事件协议和字节数据类型支持 ([ec854ca](https://github.com/puniyu/puniyu/commit/ec854caf1c2ee6e722c295cc317721c87539953e))
* **protocol:** 添加账户和适配器协议定义及Bot信息结构 ([#93](https://github.com/puniyu/puniyu/issues/93)) ([9611fba](https://github.com/puniyu/puniyu/commit/9611fba37182df82297bd1a4596473f9e81c2b5c))


### 🐛 错误修复

* **console:** 处理控制台输入错误并优雅退出 ([667ce30](https://github.com/puniyu/puniyu/commit/667ce308ab764e374b05fe27ad132c8d5ff262c2))


### 🎨 代码样式

* **code:** 项目格式化 ([dc3d850](https://github.com/puniyu/puniyu/commit/dc3d850bcca149de821bc5ec700ae98f567f4a79))


### 🔧 其他更新

* **puniyu_types:** 修改图片数据结构类型 ([b9c195b](https://github.com/puniyu/puniyu/commit/b9c195b46d86b16a1688874036ce6cfed16fe308))
* **puniyu_types:** 更新 AdapterInfo 结构体字段默认值及构造宏 ([76e8ebe](https://github.com/puniyu/puniyu/commit/76e8ebe2b6d2a60ebc935378fc4a98bdc47b414b))


### ♻️ 代码重构

* **adapter:** 重命名控制台适配器模块路径 ([5f9dcf3](https://github.com/puniyu/puniyu/commit/5f9dcf3c448225e54f1b3349b4746fb86fdf9897))
* **adapter:** 重构适配器API结构并添加模块化接口 ([#100](https://github.com/puniyu/puniyu/issues/100)) ([6558f4f](https://github.com/puniyu/puniyu/commit/6558f4faed67a89a59c4c2790a7ba986581888a5))
* **adapter:** 重构适配器信息结构与初始化逻辑 ([2e45256](https://github.com/puniyu/puniyu/commit/2e4525633031ec401f058507218cb2731ac24479))
* **adapter:** 重构适配器模块结构和依赖关系 ([3c8e01f](https://github.com/puniyu/puniyu/commit/3c8e01f3bc7046220d06e4fe9c0a919a4f9d26f2))
* **core:** 重构命令匹配器与上下文处理逻辑 ([3aca600](https://github.com/puniyu/puniyu/commit/3aca600a94a09079ecbd8e84cf51376fc1222a99))
* **element:** 修改图片文件字段类型从Vec&lt;u8&gt;到String ([744e7aa](https://github.com/puniyu/puniyu/commit/744e7aa6b8daaf43a7720ad7b65156ef63ef572e))
* **event:** 重构事件系统并重命名事件总线为事件模块 ([39093d4](https://github.com/puniyu/puniyu/commit/39093d4da202aaac142cd134d38e55e9a40b526d))
* **types:** 重构类型定义和宏实现 ([23561c9](https://github.com/puniyu/puniyu/commit/23561c9d33724d59b9b22228f4d2b192efad8faf))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_adapter bumped from 0.6.0 to 0.7.0
    * puniyu_core bumped from 0.6.0 to 0.7.0

## [0.3.0](https://github.com/puniyu/puniyu/compare/adapter-console-v0.2.13...adapter-console-v0.3.0) (2025-12-02)


### ✨ 新功能

* **macro:** 重构宏系统并增强命令参数支持 ([e8f3c23](https://github.com/puniyu/puniyu/commit/e8f3c23e42cd94fb567dce569cbe4477014300b1))
* **types:** 更新图片元素结构与消息处理逻辑 ([9b69689](https://github.com/puniyu/puniyu/commit/9b69689c679b3baa2a2d8acff99661b3e22f1766))


### 🐛 错误修复

* **puniyu_plugin:** 修正plugin宏获取crate name错误 ([b8fdbd7](https://github.com/puniyu/puniyu/commit/b8fdbd7b16371cdcbdee23d90c3075aff4cc4ee4))


### 🎨 代码样式

* **mes:** 优化消息发送日志记录格式 ([0a8336a](https://github.com/puniyu/puniyu/commit/0a8336a777a568c13d27c7e84a5952c40c0d3055))


### ♻️ 代码重构

* **command:** 修正日志格式 ([b8fdbd7](https://github.com/puniyu/puniyu/commit/b8fdbd7b16371cdcbdee23d90c3075aff4cc4ee4))
* **config:** 重构配置模块并新增适配器配置支持 ([26874a2](https://github.com/puniyu/puniyu/commit/26874a22ac9114d487ac56767927b7f1b8bbe205))
* **element:** 重构消息元素模块结构 ([2aca906](https://github.com/puniyu/puniyu/commit/2aca906d9f9d44e77753e3784539be24a16f878d))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_adapter bumped from 0.5.12 to 0.6.0
    * puniyu_core bumped from 0.5.12 to 0.6.0
    * puniyu_common bumped from 0.5.12 to 0.6.0

## [0.2.13](https://github.com/puniyu/puniyu/compare/adapter-console-v0.2.12...adapter-console-v0.2.13) (2025-11-24)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_adapter bumped from 0.5.11 to 0.5.12
    * puniyu_core bumped from 0.5.11 to 0.5.12
    * puniyu_common bumped from 0.5.11 to 0.5.12

## [0.2.12](https://github.com/puniyu/puniyu/compare/adapter-console-v0.2.11...adapter-console-v0.2.12) (2025-11-23)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_adapter bumped from 0.5.10 to 0.5.11
    * puniyu_core bumped from 0.5.10 to 0.5.11
    * puniyu_common bumped from 0.5.10 to 0.5.11

## [0.2.11](https://github.com/puniyu/puniyu/compare/adapter-console-v0.2.10...adapter-console-v0.2.11) (2025-11-23)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_adapter bumped from 0.5.9 to 0.5.10
    * puniyu_core bumped from 0.5.9 to 0.5.10
    * puniyu_common bumped from 0.5.9 to 0.5.10

## [0.2.10](https://github.com/puniyu/puniyu/compare/adapter-console-v0.2.9...adapter-console-v0.2.10) (2025-11-23)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_adapter bumped from 0.5.8 to 0.5.9
    * puniyu_core bumped from 0.5.8 to 0.5.9
    * puniyu_common bumped from 0.5.8 to 0.5.9

## [0.2.9](https://github.com/puniyu/puniyu/compare/adapter-console-v0.2.8...adapter-console-v0.2.9) (2025-11-23)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_adapter bumped from 0.5.7 to 0.5.8
    * puniyu_core bumped from 0.5.7 to 0.5.8
    * puniyu_common bumped from 0.5.7 to 0.5.8

## [0.2.8](https://github.com/puniyu/puniyu/compare/adapter-console-v0.2.7...adapter-console-v0.2.8) (2025-11-23)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_adapter bumped from 0.5.6 to 0.5.7
    * puniyu_core bumped from 0.5.6 to 0.5.7
    * puniyu_common bumped from 0.5.6 to 0.5.7

## [0.2.7](https://github.com/puniyu/puniyu/compare/adapter-console-v0.2.6...adapter-console-v0.2.7) (2025-11-23)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.5.4 to 0.5.6

## [0.2.6](https://github.com/puniyu/puniyu/compare/adapter-console-v0.2.5...adapter-console-v0.2.6) (2025-11-23)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_adapter bumped from 0.5.5 to 0.5.6
    * puniyu_core bumped from 0.5.5 to 0.5.6

## [0.2.5](https://github.com/puniyu/puniyu/compare/adapter-console-v0.2.4...adapter-console-v0.2.5) (2025-11-23)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_adapter bumped from 0.5.4 to 0.5.5
    * puniyu_core bumped from 0.5.4 to 0.5.5

## [0.2.4](https://github.com/puniyu/puniyu/compare/adapter-console-v0.2.3...adapter-console-v0.2.4) (2025-11-23)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_adapter bumped from 0.5.3 to 0.5.4
    * puniyu_core bumped from 0.5.3 to 0.5.4
    * puniyu_common bumped from 0.5.1 to 0.5.4

## [0.2.3](https://github.com/puniyu/puniyu/compare/adapter-console-v0.2.2...adapter-console-v0.2.3) (2025-11-23)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_adapter bumped from 0.5.2 to 0.5.3
    * puniyu_core bumped from 0.5.2 to 0.5.3

## [0.2.2](https://github.com/puniyu/puniyu/compare/adapter-console-v0.2.1...adapter-console-v0.2.2) (2025-11-23)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_adapter bumped from 0.5.1 to 0.5.2
    * puniyu_core bumped from 0.5.1 to 0.5.2

## [0.2.1](https://github.com/puniyu/puniyu/compare/adapter-console-v0.2.0...adapter-console-v0.2.1) (2025-11-23)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_adapter bumped from 0.5.0 to 0.5.1
    * puniyu_core bumped from 0.5.0 to 0.5.1
    * puniyu_common bumped from 0.5.0 to 0.5.1

## [0.2.0](https://github.com/puniyu/puniyu/compare/adapter-console-v0.1.12...adapter-console-v0.2.0) (2025-11-23)


### ✨ 新功能

* **adapter:** 支持配置文件读取功能 ([a9fc6e2](https://github.com/puniyu/puniyu/commit/a9fc6e2aed53370db0c78a0035c37eec53114445))


### 🐛 错误修复

* **console:** 优化控制台适配器配置与资源管理 ([31184f1](https://github.com/puniyu/puniyu/commit/31184f134328fc0b193972675e2274ea53a38864))


### ♻️ 代码重构

* **config:** 添加适配器配置系统支持 ([5358888](https://github.com/puniyu/puniyu/commit/5358888a950988a6be04c9b539bdac95c9b09b8d))
* **console:** 在账户信息中使用AVATAR_URL ([5184612](https://github.com/puniyu/puniyu/commit/5184612f4ff1a3e484cac09d9929ceb8d67233e6))
* **workspace:** 重构项目结构和依赖管理 ([520087e](https://github.com/puniyu/puniyu/commit/520087e38b49ff9e3aabe04be946038b81114b4d))
* 重构工作区crates ([#53](https://github.com/puniyu/puniyu/issues/53)) ([f55ab51](https://github.com/puniyu/puniyu/commit/f55ab519b9275c19773f16acfddff0c44db8e48c))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_adapter bumped from 0.4.1 to 0.5.0
    * puniyu_core bumped from 0.4.1 to 0.5.0
    * puniyu_common bumped from 0.4.1 to 0.5.0

## [0.1.12](https://github.com/puniyu/puniyu/compare/adapter-console-v0.1.11...adapter-console-v0.1.12) (2025-11-16)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_adapter bumped from 0.4.0 to 0.4.1
    * puniyu_core bumped from 0.4.0 to 0.4.1
    * puniyu_common bumped from 0.4.0 to 0.4.1

## [0.1.11](https://github.com/puniyu/puniyu/compare/adapter-console-v0.1.10...adapter-console-v0.1.11) (2025-11-16)


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_adapter bumped from 0.2.2 to 0.4.0
    * puniyu_core bumped from 0.3.1 to 0.4.0
    * puniyu_common bumped from 0.3.0 to 0.4.0

## [0.1.10](https://github.com/puniyu/puniyu/compare/adapter-console-v0.1.9...adapter-console-v0.1.10) (2025-11-15)


### 🔧 其他更新

* **release:** 调整发布配置并更新依赖版本 ([381ca55](https://github.com/puniyu/puniyu/commit/381ca558c7ca6b4bcc9b9386ecc228e8679f7305))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_adapter bumped from 0.2.1 to 0.2.2
    * puniyu_core bumped from 0.3.0 to 0.3.1
    * puniyu_common bumped from 0.2.0 to 0.3.0

## [0.1.9](https://github.com/puniyu/puniyu/compare/adapter-console-v0.1.8...adapter-console-v0.1.9) (2025-11-15)


### 🔧 其他更新

* **release:** 配置并发布多个 crates 包 ([1915ea1](https://github.com/puniyu/puniyu/commit/1915ea1d6da0207ab00ed0dc99f106fb289f30ef))
