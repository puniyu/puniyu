# 变更日志

## [0.7.10](https://github.com/puniyu/puniyu/compare/puniyu_command-v0.7.9...puniyu_command-v0.7.10)

### ⚙️ 杂项


- Updated the following local packages: puniyu_types, puniyu_registry, puniyu_config - ([0000000](https://github.com/puniyu/puniyu/commit/0000000))

## [0.7.9](https://github.com/puniyu/puniyu/compare/puniyu_command-v0.7.8...puniyu_command-v0.7.9) - 2026-01-12


### 🚜 重构


- *(api)* 移除不必要的解引用操作 (由 @wuliya336 提供) - ([7cbd05f](https://github.com/puniyu/puniyu/commit/7cbd05fed5ae6c5a146b0b1cc60528d6a3eb91d9))




### 贡献者

* @wuliya336
## [0.7.8](https://github.com/puniyu/puniyu/compare/puniyu_command-v0.7.7...puniyu_command-v0.7.8) - 2026-01-11


### ⚙️ 杂项



- Updated the following local packages: puniyu_types, puniyu_registry - ([0000000](https://github.com/puniyu/puniyu/commit/0000000))

## [0.7.7](https://github.com/puniyu/puniyu/compare/puniyu_command-v0.7.6...puniyu_command-v0.7.7) - 2026-01-11


### ⚙️ 杂项



- Updated the following local packages: puniyu_registry - ([0000000](https://github.com/puniyu/puniyu/commit/0000000))

## [0.7.6](https://github.com/puniyu/puniyu/compare/puniyu_command-v0.7.5...puniyu_command-v0.7.6) - 2026-01-11


### ⛰️ 新功能



- Hook功能实现 ([#112](https://github.com/puniyu/puniyu/pull/112)) (由 @wuliya336 提供) (#112) - ([1e7bbf7](https://github.com/puniyu/puniyu/commit/1e7bbf7b6d25fcfb5c8fcedc6a68f29ff6b8c12e))


### 🚜 重构


- *(command)* 重构命令处理器上下文结构 (由 @wuliya336 提供) - ([0cabc63](https://github.com/puniyu/puniyu/commit/0cabc63c70e29756a1a0e0389888576cc894fc7d))
- *(command)* 重构命令处理器实现 (由 @wuliya336 提供) - ([a3addfd](https://github.com/puniyu/puniyu/commit/a3addfded2c550d183c5e1fcb63d6014eeb97ea9))
- *(macros)* 重构proc-macro宏 ([#119](https://github.com/puniyu/puniyu/pull/119)) (由 @wuliya336 提供) (#119) - ([9942a1b](https://github.com/puniyu/puniyu/commit/9942a1baa348377af978d9224510316a0595a5f3))




### 贡献者

* @puniyu[bot]
* @wuliya336
## [0.7.5](https://github.com/puniyu/puniyu/compare/puniyu_command-v0.7.4...puniyu_command-v0.7.5) - 2026-01-11


### ⛰️ 新功能



- Hook功能实现 ([#112](https://github.com/puniyu/puniyu/pull/112)) (由 @wuliya336 提供) (#112) - ([1e7bbf7](https://github.com/puniyu/puniyu/commit/1e7bbf7b6d25fcfb5c8fcedc6a68f29ff6b8c12e))


### 🚜 重构


- *(command)* 重构命令处理器上下文结构 (由 @wuliya336 提供) - ([0cabc63](https://github.com/puniyu/puniyu/commit/0cabc63c70e29756a1a0e0389888576cc894fc7d))
- *(command)* 重构命令处理器实现 (由 @wuliya336 提供) - ([a3addfd](https://github.com/puniyu/puniyu/commit/a3addfded2c550d183c5e1fcb63d6014eeb97ea9))
- *(macros)* 重构proc-macro宏 ([#119](https://github.com/puniyu/puniyu/pull/119)) (由 @wuliya336 提供) (#119) - ([9942a1b](https://github.com/puniyu/puniyu/commit/9942a1baa348377af978d9224510316a0595a5f3))




### 贡献者

* @wuliya336
## [0.7.4](https://github.com/puniyu/puniyu/compare/puniyu_command-v0.7.3...puniyu_command-v0.7.4) - 2026-01-07


### ⛰️ 新功能


- *(adapter)* 实现server适配器 ([#99](https://github.com/puniyu/puniyu/pull/99)) (由 @shiwuliya 提供) (#99) - ([e1671ad](https://github.com/puniyu/puniyu/commit/e1671ad8ebc180f066be1ec58a508194d66e4850))
- *(core)* 自动创建插件和适配器数据目录 (由 @shiwuliya 提供) - ([881a42e](https://github.com/puniyu/puniyu/commit/881a42ece6fb13ae8ad11c94e01e9c4463a32ec4))



### 🚜 重构


- *(adapter)* 重构适配器模块结构和依赖关系 (由 @shiwuliya 提供) - ([3c8e01f](https://github.com/puniyu/puniyu/commit/3c8e01f3bc7046220d06e4fe9c0a919a4f9d26f2))
- *(adapter)* 重构适配器API结构并添加模块化接口 ([#100](https://github.com/puniyu/puniyu/pull/100)) (由 @shiwuliya 提供) (#100) - ([6558f4f](https://github.com/puniyu/puniyu/commit/6558f4faed67a89a59c4c2790a7ba986581888a5))
- *(command)* 重构命令处理系统 (由 @shiwuliya 提供) (#96) - ([549db91](https://github.com/puniyu/puniyu/commit/549db91d4bd063616c85bced71f3ea69431bf6c0))
- *(command)* 重构命令模块结构并合并matcher和handler功能 (由 @shiwuliya 提供) (#96) - ([a61cb76](https://github.com/puniyu/puniyu/commit/a61cb76138426ccb725c476905be603589cdc231))
- *(core)* 重新组织模块导入顺序并清理无用导出 (由 @shiwuliya 提供) - ([8da3556](https://github.com/puniyu/puniyu/commit/8da35564bb6e99777d30d5c48497e11358c51915))
- *(event)* 重构事件系统并重命名事件总线为事件模块 (由 @shiwuliya 提供) (#98) - ([39093d4](https://github.com/puniyu/puniyu/commit/39093d4da202aaac142cd134d38e55e9a40b526d))
- *(event)* 引入统一事件上下文和通知/请求事件 ([#31](https://github.com/puniyu/puniyu/pull/31)) (由 @shiwuliya 提供) (#31) - ([05bd056](https://github.com/puniyu/puniyu/commit/05bd05690b3fa47443f6d5982c799f88cd900f54))

- 重构工作区crates ([#53](https://github.com/puniyu/puniyu/pull/53)) (由 @shiwuliya 提供) (#53) - ([f55ab51](https://github.com/puniyu/puniyu/commit/f55ab519b9275c19773f16acfddff0c44db8e48c))
- 重构项目配置 (由 @shiwuliya 提供) - ([883dfa4](https://github.com/puniyu/puniyu/commit/883dfa4fb525d7e5c27821026d727e3d8eda8600))


### 📚 文档


- *(readme)* 添加社区QQ群链接 (由 @shiwuliya 提供) - ([60bf1e7](https://github.com/puniyu/puniyu/commit/60bf1e788718844583f6a2e6809b930b7d262775))

- Update README.md (由 @allcontributors[bot] 提供) (#17) - ([52c8b9f](https://github.com/puniyu/puniyu/commit/52c8b9fcc37d46e92147e02fff0f2bf09b9eede2))


### ⚙️ 杂项


- *(config)* 初始化配置模块并添加 README 文档 (由 @shiwuliya 提供) - ([e9099fc](https://github.com/puniyu/puniyu/commit/e9099fc66da3b14413f3851c82771f148cff5c10))
- *(workflow)* 修复发布ci (由 @shiwuliya 提供) - ([7cd84d6](https://github.com/puniyu/puniyu/commit/7cd84d6398285b00be4792942110421b71122cbe))

- 初始化仓库 (由 @shiwuliya 提供) - ([51af186](https://github.com/puniyu/puniyu/commit/51af186b465079c274bb094dc3dc8b38ba959f15))



### 贡献者

* @shiwuliya
* @puniyu[bot]
* @allcontributors[bot]
