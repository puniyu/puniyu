# Changelog

## [0.4.0](https://github.com/puniyu/puniyu/compare/v0.3.0...v0.4.0) (2025-11-15)


### ✨ 新功能

* **adapter:** 实现QQ协议适配器API接口 ([#34](https://github.com/puniyu/puniyu/issues/34)) ([18e4f9e](https://github.com/puniyu/puniyu/commit/18e4f9e7245cf8f1355d4f23eca0d2df42e8f7e5))
* **core:** 自动创建插件和适配器数据目录 ([881a42e](https://github.com/puniyu/puniyu/commit/881a42ece6fb13ae8ad11c94e01e9c4463a32ec4))


### 🔧 其他更新

* **config:** 初始化配置模块并添加 README 文档 ([6109f15](https://github.com/puniyu/puniyu/commit/6109f151b73d1ad24c5237f5602aad40a7fbbba4))
* **config:** 初始化配置模块并添加 README 文档 ([e9099fc](https://github.com/puniyu/puniyu/commit/e9099fc66da3b14413f3851c82771f148cff5c10))
* release main ([b22b2f0](https://github.com/puniyu/puniyu/commit/b22b2f017c88290346428c229c975cc570bc70d1))
* release main ([be9bdfe](https://github.com/puniyu/puniyu/commit/be9bdfe1fcee37185d800f1cf0bd5758d83776e1))
* **release:** 调整发布配置并更新依赖版本 ([381ca55](https://github.com/puniyu/puniyu/commit/381ca558c7ca6b4bcc9b9386ecc228e8679f7305))


### ♻️ 代码重构

* **adapter:** 简化适配器加载逻辑并移除ABI版本检查 ([30104ed](https://github.com/puniyu/puniyu/commit/30104edcd5c1e81ffb87a4da6718bbc0399ff941))
* **adapter:** 重构适配器模块结构和依赖关系 ([1cedfac](https://github.com/puniyu/puniyu/commit/1cedfac70a93d071b25ea2721df7c9f41123e1bf))
* **core:** 重构消息事件和适配器API错误类型 ([e0aeeb1](https://github.com/puniyu/puniyu/commit/e0aeeb19fdff296beece58fb1cc5d8ebd36abf26))
* **element:** 移除冗余的type字段并优化文件元素结构 ([2e659d5](https://github.com/puniyu/puniyu/commit/2e659d59997543d1dac50f614ba847b6477ef0ab))
* **event:** 引入统一事件上下文和通知/请求事件 ([#31](https://github.com/puniyu/puniyu/issues/31)) ([05bd056](https://github.com/puniyu/puniyu/commit/05bd05690b3fa47443f6d5982c799f88cd900f54))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_element bumped from 0.3.0 to 0.4.0
    * puniyu_adapter_api bumped from 0.2.1 to 0.2.2
    * puniyu_sender bumped from 0.1.10 to 0.1.11
    * puniyu_contact bumped from 0.1.10 to 0.1.11
    * puniyu_config bumped from 0.3.0 to 0.4.0

## [0.3.0](https://github.com/puniyu/puniyu/compare/v0.2.0...v0.3.0) (2025-11-15)


### ✨ 新功能

* **adapter:** 实现QQ协议适配器API接口 ([#34](https://github.com/puniyu/puniyu/issues/34)) ([18e4f9e](https://github.com/puniyu/puniyu/commit/18e4f9e7245cf8f1355d4f23eca0d2df42e8f7e5))
* **core:** 自动创建插件和适配器数据目录 ([881a42e](https://github.com/puniyu/puniyu/commit/881a42ece6fb13ae8ad11c94e01e9c4463a32ec4))


### 🔧 其他更新

* **config:** 初始化配置模块并添加 README 文档 ([6109f15](https://github.com/puniyu/puniyu/commit/6109f151b73d1ad24c5237f5602aad40a7fbbba4))
* **config:** 初始化配置模块并添加 README 文档 ([e9099fc](https://github.com/puniyu/puniyu/commit/e9099fc66da3b14413f3851c82771f148cff5c10))
* release main ([be9bdfe](https://github.com/puniyu/puniyu/commit/be9bdfe1fcee37185d800f1cf0bd5758d83776e1))
* **release:** 调整发布配置并更新依赖版本 ([381ca55](https://github.com/puniyu/puniyu/commit/381ca558c7ca6b4bcc9b9386ecc228e8679f7305))


### ♻️ 代码重构

* **adapter:** 简化适配器加载逻辑并移除ABI版本检查 ([30104ed](https://github.com/puniyu/puniyu/commit/30104edcd5c1e81ffb87a4da6718bbc0399ff941))
* **adapter:** 重构适配器模块结构和依赖关系 ([1cedfac](https://github.com/puniyu/puniyu/commit/1cedfac70a93d071b25ea2721df7c9f41123e1bf))
* **core:** 重构消息事件和适配器API错误类型 ([e0aeeb1](https://github.com/puniyu/puniyu/commit/e0aeeb19fdff296beece58fb1cc5d8ebd36abf26))
* **element:** 移除冗余的type字段并优化文件元素结构 ([2e659d5](https://github.com/puniyu/puniyu/commit/2e659d59997543d1dac50f614ba847b6477ef0ab))
* **event:** 引入统一事件上下文和通知/请求事件 ([#31](https://github.com/puniyu/puniyu/issues/31)) ([05bd056](https://github.com/puniyu/puniyu/commit/05bd05690b3fa47443f6d5982c799f88cd900f54))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_element bumped from 0.2.0 to 0.3.0
    * puniyu_adapter_api bumped from 0.2.0 to 0.2.1
    * puniyu_sender bumped from 0.1.9 to 0.1.10
    * puniyu_contact bumped from 0.1.9 to 0.1.10
    * puniyu_config bumped from 0.2.0 to 0.3.0

## [0.2.0](https://github.com/puniyu/puniyu/compare/event-v0.1.8...event-v0.2.0) (2025-11-15)


### ✨ 新功能

* **adapter:** 实现QQ协议适配器API接口 ([#34](https://github.com/puniyu/puniyu/issues/34)) ([18e4f9e](https://github.com/puniyu/puniyu/commit/18e4f9e7245cf8f1355d4f23eca0d2df42e8f7e5))
* **core:** 自动创建插件和适配器数据目录 ([881a42e](https://github.com/puniyu/puniyu/commit/881a42ece6fb13ae8ad11c94e01e9c4463a32ec4))


### 🔧 其他更新

* **config:** 初始化配置模块并添加 README 文档 ([6109f15](https://github.com/puniyu/puniyu/commit/6109f151b73d1ad24c5237f5602aad40a7fbbba4))
* **config:** 初始化配置模块并添加 README 文档 ([e9099fc](https://github.com/puniyu/puniyu/commit/e9099fc66da3b14413f3851c82771f148cff5c10))


### ♻️ 代码重构

* **adapter:** 简化适配器加载逻辑并移除ABI版本检查 ([30104ed](https://github.com/puniyu/puniyu/commit/30104edcd5c1e81ffb87a4da6718bbc0399ff941))
* **adapter:** 重构适配器模块结构和依赖关系 ([1cedfac](https://github.com/puniyu/puniyu/commit/1cedfac70a93d071b25ea2721df7c9f41123e1bf))
* **core:** 重构消息事件和适配器API错误类型 ([e0aeeb1](https://github.com/puniyu/puniyu/commit/e0aeeb19fdff296beece58fb1cc5d8ebd36abf26))
* **element:** 移除冗余的type字段并优化文件元素结构 ([2e659d5](https://github.com/puniyu/puniyu/commit/2e659d59997543d1dac50f614ba847b6477ef0ab))
* **event:** 引入统一事件上下文和通知/请求事件 ([#31](https://github.com/puniyu/puniyu/issues/31)) ([05bd056](https://github.com/puniyu/puniyu/commit/05bd05690b3fa47443f6d5982c799f88cd900f54))
