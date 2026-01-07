# Changelog
# 变更日志
## [0.2.1](https://github.com/puniyu/puniyu/compare/puniyu_protocol-v0.2.0...puniyu_protocol-v0.2.1) - 2026-01-07


### ⛰️ 新功能


- *(adapter)* 实现server适配器 ([#99](https://github.com/puniyu/puniyu/pull/99)) (由 @shiwuliya 提供) (#99) - ([e1671ad](https://github.com/puniyu/puniyu/commit/e1671ad8ebc180f066be1ec58a508194d66e4850))
- *(element)* 重构消息元素类型和协议定义 (由 @shiwuliya 提供) (#91) - ([e0d01c2](https://github.com/puniyu/puniyu/commit/e0d01c24f48d68a655cb19ed909938e4cd433a1c))
- *(protocol)* 添加账户和适配器协议定义及Bot信息结构 ([#93](https://github.com/puniyu/puniyu/pull/93)) (由 @shiwuliya 提供) (#93) - ([9611fba](https://github.com/puniyu/puniyu/commit/9611fba37182df82297bd1a4596473f9e81c2b5c))
- *(protocol)* 实现事件协议和字节数据类型支持 (由 @shiwuliya 提供) (#91) - ([ec854ca](https://github.com/puniyu/puniyu/commit/ec854caf1c2ee6e722c295cc317721c87539953e))
- *(protocol)* 添加联系人和发送者协议支持并重构元素处理 (由 @shiwuliya 提供) (#91) - ([e3e6bba](https://github.com/puniyu/puniyu/commit/e3e6bbabb68d714ee01c1cc482e1055a84d88222))
- *(protocol)* 添加 puniyu 协议库用于数据交换 (由 @shiwuliya 提供) (#91) - ([46ab2dd](https://github.com/puniyu/puniyu/commit/46ab2dd515632d4741febf89dea7b250f30d479f))



### 🐛 Bug 修复


- *(event)* 事件类型转换实现 (由 @shiwuliya 提供) - ([a91e071](https://github.com/puniyu/puniyu/commit/a91e0711627d4dce672ed038d7866d81b33d1c1c))



### 🚜 重构


- *(adapter)* 重构适配器模块结构和依赖关系 (由 @shiwuliya 提供) - ([3c8e01f](https://github.com/puniyu/puniyu/commit/3c8e01f3bc7046220d06e4fe9c0a919a4f9d26f2))
- *(adapter)* 重构适配器API结构并添加模块化接口 ([#100](https://github.com/puniyu/puniyu/pull/100)) (由 @shiwuliya 提供) (#100) - ([6558f4f](https://github.com/puniyu/puniyu/commit/6558f4faed67a89a59c4c2790a7ba986581888a5))
- *(adapter)* 将时间类型从 SystemTime 替换为 OffsetDateTime (由 @shiwuliya 提供) - ([3155084](https://github.com/puniyu/puniyu/commit/3155084efb517ec67e9ca4ce31d83fa008d6d0ea))
- *(command)* 重构命令处理系统 (由 @shiwuliya 提供) (#96) - ([549db91](https://github.com/puniyu/puniyu/commit/549db91d4bd063616c85bced71f3ea69431bf6c0))
- *(core)* 重新组织模块导入顺序并清理无用导出 (由 @shiwuliya 提供) - ([8da3556](https://github.com/puniyu/puniyu/commit/8da35564bb6e99777d30d5c48497e11358c51915))
- *(protocol)* 重构事件协议结构 (由 @shiwuliya 提供) - ([09930b7](https://github.com/puniyu/puniyu/commit/09930b7d6ca1b1b5e2d9e04726eede0c3c02ad44))
- *(protocol)* 优化事件处理和类型转换逻辑 (由 @shiwuliya 提供) (#91) - ([e37a28f](https://github.com/puniyu/puniyu/commit/e37a28f88f88c9714a2035d8e7a1ea7e99139d9d))
- *(types)* 重构类型定义和宏实现 (由 @shiwuliya 提供) (#91) - ([23561c9](https://github.com/puniyu/puniyu/commit/23561c9d33724d59b9b22228f4d2b192efad8faf))

- 重构项目配置 (由 @shiwuliya 提供) - ([883dfa4](https://github.com/puniyu/puniyu/commit/883dfa4fb525d7e5c27821026d727e3d8eda8600))


### 📚 文档


- *(readme)* 添加社区QQ群链接 (由 @shiwuliya 提供) - ([60bf1e7](https://github.com/puniyu/puniyu/commit/60bf1e788718844583f6a2e6809b930b7d262775))

- Update README.md (由 @allcontributors[bot] 提供) (#17) - ([52c8b9f](https://github.com/puniyu/puniyu/commit/52c8b9fcc37d46e92147e02fff0f2bf09b9eede2))


### 🎨 样式


- *(code)* 项目格式化 (由 @shiwuliya 提供) - ([dc3d850](https://github.com/puniyu/puniyu/commit/dc3d850bcca149de821bc5ec700ae98f567f4a79))



### 🧪 测试


- *(adapter)* 添加适配器类型枚举的单元测试 (由 @shiwuliya 提供) - ([31fd2b3](https://github.com/puniyu/puniyu/commit/31fd2b3b8abd6af4b633d620046f05c3385838e4))



### ⚙️ 杂项


- *(puniyu_types)* 更新 AdapterInfo 结构体字段默认值及构造宏 (由 @shiwuliya 提供) (#91) - ([76e8ebe](https://github.com/puniyu/puniyu/commit/76e8ebe2b6d2a60ebc935378fc4a98bdc47b414b))

- 初始化仓库 (由 @shiwuliya 提供) - ([51af186](https://github.com/puniyu/puniyu/commit/51af186b465079c274bb094dc3dc8b38ba959f15))



### 贡献者

* @puniyu[bot]
* @shiwuliya
* @allcontributors[bot]

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
