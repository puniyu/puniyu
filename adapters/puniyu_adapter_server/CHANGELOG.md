# 变更日志

## [0.3.7](https://github.com/puniyu/puniyu/compare/puniyu_adapter_server-v0.3.6...puniyu_adapter_server-v0.3.7)

### 🚜 重构


- *(adapter)* 重构适配器注册相关内容 - ([cdec0b9](https://github.com/puniyu/puniyu/commit/cdec0b9e002e12aa6effb6dfda8dc3b331a1fec1))
- *(protocol)* 将 puniyu_protocol 重命名为 puniyu_probuff - ([b68d4ec](https://github.com/puniyu/puniyu/commit/b68d4ecc7f256f9deae6ca0f44bdcbc165d65eaf))
- *(registry)* 重构注册表 - ([4e93d6c](https://github.com/puniyu/puniyu/commit/4e93d6c44c888309cd236a6d92a38e543e943e2f))
- 优化全局注册表, 减少调用时开销 ([#144](https://github.com/puniyu/puniyu/pull/144)) - ([1447162](https://github.com/puniyu/puniyu/commit/1447162841cbebfba06e12eaad9fea263aa0436f))

### ⚙️ 杂项


- *(workspace)* 移除release-please配置文件并优化Cargo.toml结构 - ([db957c3](https://github.com/puniyu/puniyu/commit/db957c3939f38d30da5cc8807aed0e154fe23a52))

## [0.3.6](https://github.com/puniyu/puniyu/compare/puniyu_adapter_server-v0.3.5...puniyu_adapter_server-v0.3.6) - 2026-01-12


### 🚜 重构


- *(api)* 移除不必要的解引用操作 (由 @wuliya336 提供) - ([7cbd05f](https://github.com/puniyu/puniyu/commit/7cbd05fed5ae6c5a146b0b1cc60528d6a3eb91d9))




### 贡献者

* @wuliya336
## [0.3.5](https://github.com/puniyu/puniyu/compare/puniyu_adapter_server-v0.3.4...puniyu_adapter_server-v0.3.5) - 2026-01-11


### ⚙️ 杂项



- Updated the following local packages: puniyu_adapter, puniyu_core, puniyu_probuff - ([0000000](https://github.com/puniyu/puniyu/commit/0000000))

## [0.3.4](https://github.com/puniyu/puniyu/compare/puniyu_adapter_server-v0.3.3...puniyu_adapter_server-v0.3.4) - 2026-01-11


### ⛰️ 新功能


- *(adapter)* 实现适配器服务器的消息转发功能 (由 @wuliya336 提供) - ([e240974](https://github.com/puniyu/puniyu/commit/e2409749a42e40eeff725bb98d3c5d987d324320))



### 🐛 Bug 修复


- *(plugin)* 修复插件系统钩子注册 (由 @wuliya336 提供) - ([dfd0d6e](https://github.com/puniyu/puniyu/commit/dfd0d6e21f14ddf5e6c1a5468f33ce10d0606fdb))



### 🚜 重构


- *(server)* 移除事件服务器钩子函数 (由 @wuliya336 提供) - ([68262d4](https://github.com/puniyu/puniyu/commit/68262d4f71af960eb3af2003fe44df879b6c7ab9))




### 贡献者

* @puniyu[bot]
* @wuliya336
## [0.3.3](https://github.com/puniyu/puniyu/compare/puniyu_adapter_server-v0.3.2...puniyu_adapter_server-v0.3.3) - 2026-01-11


### ⛰️ 新功能


- *(adapter)* 实现适配器服务器的消息转发功能 (由 @wuliya336 提供) - ([e240974](https://github.com/puniyu/puniyu/commit/e2409749a42e40eeff725bb98d3c5d987d324320))



### 🐛 Bug 修复


- *(plugin)* 修复插件系统钩子注册 (由 @wuliya336 提供) - ([dfd0d6e](https://github.com/puniyu/puniyu/commit/dfd0d6e21f14ddf5e6c1a5468f33ce10d0606fdb))



### 🚜 重构


- *(server)* 移除事件服务器钩子函数 (由 @wuliya336 提供) - ([68262d4](https://github.com/puniyu/puniyu/commit/68262d4f71af960eb3af2003fe44df879b6c7ab9))




### 贡献者

* @wuliya336
