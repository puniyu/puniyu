# 变更日志

## [0.7.7](https://github.com/puniyu/puniyu/compare/puniyu_macros-v0.7.6...puniyu_macros-v0.7.7)

### 🐛 Bug 修复


- *(command)* 添加命令返回类型验证并优化消息事件处理 - ([8db2173](https://github.com/puniyu/puniyu/commit/8db21733d5fe99034f285bb4035e4dbf30340eee))

### 🚜 重构


- *(adapter)* 重构适配器注册相关内容 - ([cdec0b9](https://github.com/puniyu/puniyu/commit/cdec0b9e002e12aa6effb6dfda8dc3b331a1fec1))
- *(core)* 重构部分核心实现 ([#150](https://github.com/puniyu/puniyu/pull/150)) - ([e06459e](https://github.com/puniyu/puniyu/commit/e06459e40f1ec37f8e0de7427e6905c9bb295d5e))
- *(plugin)* 统一插件初始化返回类型为HandlerResult - ([31c1e58](https://github.com/puniyu/puniyu/commit/31c1e584885121f6464e761d7008ce1548460a57))
- *(registry)* 重构注册表 - ([4e93d6c](https://github.com/puniyu/puniyu/commit/4e93d6c44c888309cd236a6d92a38e543e943e2f))
- 优化全局注册表, 减少调用时开销 ([#144](https://github.com/puniyu/puniyu/pull/144)) - ([1447162](https://github.com/puniyu/puniyu/commit/1447162841cbebfba06e12eaad9fea263aa0436f))

## [0.7.6](https://github.com/puniyu/puniyu/compare/puniyu_macros-v0.7.5...puniyu_macros-v0.7.6) - 2026-01-12


### 🚜 重构


- *(api)* 移除不必要的解引用操作 (由 @wuliya336 提供) - ([7cbd05f](https://github.com/puniyu/puniyu/commit/7cbd05fed5ae6c5a146b0b1cc60528d6a3eb91d9))




### 贡献者

* @wuliya336
## [0.7.5](https://github.com/puniyu/puniyu/compare/puniyu_macros-v0.7.4...puniyu_macros-v0.7.5) - 2026-01-11


### ⛰️ 新功能



- Hook功能实现 ([#112](https://github.com/puniyu/puniyu/pull/112)) (由 @wuliya336 提供) (#112) - ([1e7bbf7](https://github.com/puniyu/puniyu/commit/1e7bbf7b6d25fcfb5c8fcedc6a68f29ff6b8c12e))


### 🐛 Bug 修复


- *(macros)* 修复过程宏参数类型 (由 @wuliya336 提供) - ([4091dc4](https://github.com/puniyu/puniyu/commit/4091dc4c57c928a3484fbb72b90626c6338840f3))



### 🚜 重构


- *(command)* 重构命令处理器上下文结构 (由 @wuliya336 提供) - ([0cabc63](https://github.com/puniyu/puniyu/commit/0cabc63c70e29756a1a0e0389888576cc894fc7d))
- *(command)* 重构命令处理器实现 (由 @wuliya336 提供) - ([a3addfd](https://github.com/puniyu/puniyu/commit/a3addfded2c550d183c5e1fcb63d6014eeb97ea9))
- *(macros)* 重构proc-macro宏 ([#119](https://github.com/puniyu/puniyu/pull/119)) (由 @wuliya336 提供) (#119) - ([9942a1b](https://github.com/puniyu/puniyu/commit/9942a1baa348377af978d9224510316a0595a5f3))




### 贡献者

* @wuliya336
## [0.7.4](https://github.com/puniyu/puniyu/compare/puniyu_macros-v0.7.3...puniyu_macros-v0.7.4) - 2026-01-07


### ⚙️ 杂项


- *(workflow)* 修复发布ci (由 @shiwuliya 提供) - ([7cd84d6](https://github.com/puniyu/puniyu/commit/7cd84d6398285b00be4792942110421b71122cbe))




### 贡献者

* @shiwuliya
