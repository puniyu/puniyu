# 变更日志

## [0.8.0]

### ⛰️ 新功能


- *(core)* 添加配置注册功能并优化初始化逻辑 - ([5593859](https://github.com/puniyu/puniyu/commit/5593859d698f03f01b5e3dd1685984bdfe14c96c))
- 添加命令解析器和配置管理功能 - ([075d485](https://github.com/puniyu/puniyu/commit/075d48589a9da7ec45fbd88c60cbe5a3435a3163))

### 🐛 Bug 修复


- *(command)* 添加命令返回类型验证并优化消息事件处理 - ([8db2173](https://github.com/puniyu/puniyu/commit/8db21733d5fe99034f285bb4035e4dbf30340eee))
- *(plugin)* 修复插件宏中空函数体的返回类型验证 - ([da7f237](https://github.com/puniyu/puniyu/commit/da7f2379b0f7860c04cc0e28df423e534b48b60b))

### 🚜 重构


- *(adapter)* 统一HandlerResult类型并优化API实现 - ([3926f29](https://github.com/puniyu/puniyu/commit/3926f29742bb000b8b861c35205257b85be9017e))
- *(build)* 重构构建系统并添加插件支持功能 - ([d77fb08](https://github.com/puniyu/puniyu/commit/d77fb084b9da1562475845f8b1d7689df098fc8d))
- *(core)* 重构整个框架的核心实现 ([#165](https://github.com/puniyu/puniyu/pull/165)) - ([9482b34](https://github.com/puniyu/puniyu/commit/9482b34a76fecf05b3475aecf01df93d9e8994ff))
- *(core)* 重构部分核心实现 ([#150](https://github.com/puniyu/puniyu/pull/150)) - ([e06459e](https://github.com/puniyu/puniyu/commit/e06459e40f1ec37f8e0de7427e6905c9bb295d5e))
- 重构事件模块 ([#180](https://github.com/puniyu/puniyu/pull/180)) - ([bfbb9a7](https://github.com/puniyu/puniyu/commit/bfbb9a73d6c5cdeaa2a139cdb353988e28d16fac))
- 重构事件/适配器api/上下文 ([#178](https://github.com/puniyu/puniyu/pull/178)) - ([fe74041](https://github.com/puniyu/puniyu/commit/fe74041525fac5102ed96ed2a7389ecd1a27f7ed))
- 优化全局注册表, 减少调用时开销 ([#144](https://github.com/puniyu/puniyu/pull/144)) - ([1447162](https://github.com/puniyu/puniyu/commit/1447162841cbebfba06e12eaad9fea263aa0436f))

### ⚙️ 杂项


- *(workspace)* 移除release-please配置文件并优化Cargo.toml结构 - ([db957c3](https://github.com/puniyu/puniyu/commit/db957c3939f38d30da5cc8807aed0e154fe23a52))

## [0.2.5](https://github.com/puniyu/puniyu/compare/plugin-basic-v0.2.4...plugin-basic-v0.2.5) - 2026-01-12


### 🚜 重构


- *(api)* 移除不必要的解引用操作 (由 @wuliya336 提供) - ([7cbd05f](https://github.com/puniyu/puniyu/commit/7cbd05fed5ae6c5a146b0b1cc60528d6a3eb91d9))




### 贡献者

* @wuliya336
## [0.2.4](https://github.com/puniyu/puniyu/compare/plugin-basic-v0.2.3...plugin-basic-v0.2.4) - 2026-01-11


### ⚙️ 杂项



- Updated the following local packages: puniyu_core, puniyu_plugin - ([0000000](https://github.com/puniyu/puniyu/commit/0000000))

## [0.2.3](https://github.com/puniyu/puniyu/compare/plugin-basic-v0.2.2...plugin-basic-v0.2.3) - 2026-01-11


### ⛰️ 新功能



- Hook功能实现 ([#112](https://github.com/puniyu/puniyu/pull/112)) (由 @wuliya336 提供) (#112) - ([1e7bbf7](https://github.com/puniyu/puniyu/commit/1e7bbf7b6d25fcfb5c8fcedc6a68f29ff6b8c12e))


### 🚜 重构


- *(command)* 重构命令处理器上下文结构 (由 @wuliya336 提供) - ([0cabc63](https://github.com/puniyu/puniyu/commit/0cabc63c70e29756a1a0e0389888576cc894fc7d))
- *(macros)* 重构proc-macro宏 ([#119](https://github.com/puniyu/puniyu/pull/119)) (由 @wuliya336 提供) (#119) - ([9942a1b](https://github.com/puniyu/puniyu/commit/9942a1baa348377af978d9224510316a0595a5f3))




### 贡献者

* @puniyu[bot]
* @wuliya336
## [0.2.2](https://github.com/puniyu/puniyu/compare/plugin-basic-v0.2.1...plugin-basic-v0.2.2) - 2026-01-11


### ⛰️ 新功能



- Hook功能实现 ([#112](https://github.com/puniyu/puniyu/pull/112)) (由 @wuliya336 提供) (#112) - ([1e7bbf7](https://github.com/puniyu/puniyu/commit/1e7bbf7b6d25fcfb5c8fcedc6a68f29ff6b8c12e))


### 🚜 重构


- *(command)* 重构命令处理器上下文结构 (由 @wuliya336 提供) - ([0cabc63](https://github.com/puniyu/puniyu/commit/0cabc63c70e29756a1a0e0389888576cc894fc7d))
- *(macros)* 重构proc-macro宏 ([#119](https://github.com/puniyu/puniyu/pull/119)) (由 @wuliya336 提供) (#119) - ([9942a1b](https://github.com/puniyu/puniyu/commit/9942a1baa348377af978d9224510316a0595a5f3))




### 贡献者

* @wuliya336
