# Changelog

## 0.4.2 (2025-11-22)


### 🔧 其他更新

* **deps:** 更新 tokio 依赖并启用完整特性 ([1b717fa](https://github.com/puniyu/puniyu/commit/1b717faacc17a23de1ec0a7856744c80d97b1a85))


### ♻️ 代码重构

* **adapter:** 优化 Avatar 类型实现并添加服务器 logo 接口 ([952c18b](https://github.com/puniyu/puniyu/commit/952c18b3008a5e31fd00127dc6d2fb55568c9796))
* **api:** 简化logo路由注册方式 ([4337b56](https://github.com/puniyu/puniyu/commit/4337b569d4abf62f53366710e19c0f049f898938))
* 重构工作区crates ([#53](https://github.com/puniyu/puniyu/issues/53)) ([f55ab51](https://github.com/puniyu/puniyu/commit/f55ab519b9275c19773f16acfddff0c44db8e48c))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * puniyu_common bumped from 0.4.1 to 0.4.2
    * puniyu_types bumped from 0.4.1 to 0.4.2
    * puniyu_registry bumped from 0.4.1 to 0.4.2

## [0.4.1](https://github.com/puniyu/puniyu/compare/server-v0.4.0...server-v0.4.1) (2025-11-16)


### 🔧 其他更新

* **server:** Synchronize puniyu versions

## [0.4.0](https://github.com/puniyu/puniyu/compare/server-v0.2.0...server-v0.4.0) (2025-11-16)


### 🔧 其他更新

* **server:** Synchronize puniyu versions

## [0.2.0](https://github.com/puniyu/puniyu/compare/server-v0.1.8...server-v0.2.0) (2025-11-15)


### ✨ 新功能

* **server:** 条件编译控制日志初始化 ([859c86e](https://github.com/puniyu/puniyu/commit/859c86e8d0e2047643b7c41931055950ab96c839))
* **server:** 添加 HTTP 服务器模块并初始化配置 ([7e83259](https://github.com/puniyu/puniyu/commit/7e832595521a21081530a8c9c318d416e747ee0b))
* **server:** 添加服务器停止功能并优化启动逻辑 ([7324370](https://github.com/puniyu/puniyu/commit/7324370202ead7e9902c56964ebf33d683ed136b))


### 🔧 其他更新

* **config:** 初始化配置模块并添加 README 文档 ([fd3d0e8](https://github.com/puniyu/puniyu/commit/fd3d0e851964cc27af11c500e2c5b0d6214a7bd0))


### ♻️ 代码重构

* **adapter:** 重构适配器模块结构和依赖关系 ([1cedfac](https://github.com/puniyu/puniyu/commit/1cedfac70a93d071b25ea2721df7c9f41123e1bf))
* **server:** 移除日志功能的特性标志并初始化主函数中的日志 ([156fd0e](https://github.com/puniyu/puniyu/commit/156fd0e3dbf637d6dd5ee0630e68dd9dfbd27744))
