# Changelog

## [0.2.0](https://github.com/puniyu/puniyu/compare/types-v0.1.0...types-v0.2.0) (2026-01-06)


### ✨ 新功能

* **adapter:** 实现server适配器 ([#99](https://github.com/puniyu/puniyu/issues/99)) ([e1671ad](https://github.com/puniyu/puniyu/commit/e1671ad8ebc180f066be1ec58a508194d66e4850))
* **element:** 重构消息元素类型和协议定义 ([e0d01c2](https://github.com/puniyu/puniyu/commit/e0d01c24f48d68a655cb19ed909938e4cd433a1c))
* **protocol:** 实现事件协议和字节数据类型支持 ([ec854ca](https://github.com/puniyu/puniyu/commit/ec854caf1c2ee6e722c295cc317721c87539953e))
* **protocol:** 添加 puniyu 协议库用于数据交换 ([46ab2dd](https://github.com/puniyu/puniyu/commit/46ab2dd515632d4741febf89dea7b250f30d479f))
* **protocol:** 添加联系人和发送者协议支持并重构元素处理 ([e3e6bba](https://github.com/puniyu/puniyu/commit/e3e6bbabb68d714ee01c1cc482e1055a84d88222))
* **protocol:** 添加账户和适配器协议定义及Bot信息结构 ([#93](https://github.com/puniyu/puniyu/issues/93)) ([9611fba](https://github.com/puniyu/puniyu/commit/9611fba37182df82297bd1a4596473f9e81c2b5c))


### 🐛 错误修复

* **event:** 事件类型转换实现 ([a91e071](https://github.com/puniyu/puniyu/commit/a91e0711627d4dce672ed038d7866d81b33d1c1c))


### 🎨 代码样式

* **code:** 项目格式化 ([dc3d850](https://github.com/puniyu/puniyu/commit/dc3d850bcca149de821bc5ec700ae98f567f4a79))


### 🔧 其他更新

* **puniyu_types:** 更新 AdapterInfo 结构体字段默认值及构造宏 ([76e8ebe](https://github.com/puniyu/puniyu/commit/76e8ebe2b6d2a60ebc935378fc4a98bdc47b414b))


### ♻️ 代码重构

* **adapter:** 将时间类型从 SystemTime 替换为 OffsetDateTime ([3155084](https://github.com/puniyu/puniyu/commit/3155084efb517ec67e9ca4ce31d83fa008d6d0ea))
* **adapter:** 重构适配器API结构并添加模块化接口 ([#100](https://github.com/puniyu/puniyu/issues/100)) ([6558f4f](https://github.com/puniyu/puniyu/commit/6558f4faed67a89a59c4c2790a7ba986581888a5))
* **adapter:** 重构适配器模块结构和依赖关系 ([3c8e01f](https://github.com/puniyu/puniyu/commit/3c8e01f3bc7046220d06e4fe9c0a919a4f9d26f2))
* **command:** 重构命令处理系统 ([549db91](https://github.com/puniyu/puniyu/commit/549db91d4bd063616c85bced71f3ea69431bf6c0))
* **protocol:** 优化事件处理和类型转换逻辑 ([e37a28f](https://github.com/puniyu/puniyu/commit/e37a28f88f88c9714a2035d8e7a1ea7e99139d9d))
* **protocol:** 重构事件协议结构 ([09930b7](https://github.com/puniyu/puniyu/commit/09930b7d6ca1b1b5e2d9e04726eede0c3c02ad44))
* **types:** 重构类型定义和宏实现 ([23561c9](https://github.com/puniyu/puniyu/commit/23561c9d33724d59b9b22228f4d2b192efad8faf))


### ✅ 测试相关

* **adapter:** 添加适配器类型枚举的单元测试 ([31fd2b3](https://github.com/puniyu/puniyu/commit/31fd2b3b8abd6af4b633d620046f05c3385838e4))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_types bumped from 0.6.0 to 0.7.0
