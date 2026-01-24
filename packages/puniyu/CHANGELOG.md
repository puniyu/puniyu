# 变更日志

## [0.7.9](https://github.com/puniyu/puniyu/compare/puniyu-v0.7.8...puniyu-v0.7.9)

### ⛰️ 新功能


- *(adapter)* 实现server适配器 ([#99](https://github.com/puniyu/puniyu/pull/99)) - ([e1671ad](https://github.com/puniyu/puniyu/commit/e1671ad8ebc180f066be1ec58a508194d66e4850))
- *(config)* 添加全局命令前缀配置并优化模块结构 - ([e0e5bb5](https://github.com/puniyu/puniyu/commit/e0e5bb5ac9eb24a37189a3d50b42d3db8db58dde))
- *(core)* 将 puniyu_logger 设置为可选依赖并更新版本获取逻辑 - ([f49677a](https://github.com/puniyu/puniyu/commit/f49677ad1bcf2c5b64ff699241c427a7a7300cc0))
- *(plugin)* 新增服务端插件支持 - ([7f15acf](https://github.com/puniyu/puniyu/commit/7f15acf148d002e33ef246b3a65a08866a44393f))
- *(plugin)* 添加基础插件及状态命令功能 - ([2503696](https://github.com/puniyu/puniyu/commit/2503696fa293909244f5110e8504ae00bf8b962b))
- *(protocol)* 添加账户和适配器协议定义及Bot信息结构 ([#93](https://github.com/puniyu/puniyu/pull/93)) - ([9611fba](https://github.com/puniyu/puniyu/commit/9611fba37182df82297bd1a4596473f9e81c2b5c))
- *(types)* 更新图片元素结构与消息处理逻辑 - ([9b69689](https://github.com/puniyu/puniyu/commit/9b69689c679b3baa2a2d8acff99661b3e22f1766))
- Hook功能实现 ([#112](https://github.com/puniyu/puniyu/pull/112)) - ([1e7bbf7](https://github.com/puniyu/puniyu/commit/1e7bbf7b6d25fcfb5c8fcedc6a68f29ff6b8c12e))

### 🐛 Bug 修复


- *(context)* 更新回复方法为泛型参数 - ([fc41520](https://github.com/puniyu/puniyu/commit/fc41520a57b357ba1516cae99490a082afbbd9bb))

### 🚜 重构


- *(account)* 重构 AccountInfo 的结构体 - ([0ae3ee2](https://github.com/puniyu/puniyu/commit/0ae3ee2ff7a242a6402458a124604676a191c2a8))
- *(adapter)* 重构适配器模块结构和依赖关系 - ([3c8e01f](https://github.com/puniyu/puniyu/commit/3c8e01f3bc7046220d06e4fe9c0a919a4f9d26f2))
- *(adapter)* 重命名控制台适配器模块路径 - ([5f9dcf3](https://github.com/puniyu/puniyu/commit/5f9dcf3c448225e54f1b3349b4746fb86fdf9897))
- *(app)* 重构应用构建器和适配器加载逻辑 - ([58ddb3f](https://github.com/puniyu/puniyu/commit/58ddb3f0decf9b50b2e1270a8da6dc914a88dfd0))
- *(core)* 重新组织模块导入顺序并清理无用导出 - ([8da3556](https://github.com/puniyu/puniyu/commit/8da35564bb6e99777d30d5c48497e11358c51915))
- *(core)* 移动logo资源到core包并优化构建配置 - ([4e2d787](https://github.com/puniyu/puniyu/commit/4e2d787dd92318b45d1128d11ffc26c2451729c3))
- *(macros)* 重构proc-macro宏 ([#119](https://github.com/puniyu/puniyu/pull/119)) - ([9942a1b](https://github.com/puniyu/puniyu/commit/9942a1baa348377af978d9224510316a0595a5f3))
- *(project)* 重构项目结构 - ([b1b389f](https://github.com/puniyu/puniyu/commit/b1b389f25dae5899b49133be88ba348930117972))
- *(workspace)* 重构项目结构和依赖管理 - ([520087e](https://github.com/puniyu/puniyu/commit/520087e38b49ff9e3aabe04be946038b81114b4d))
- 重构工作区crates ([#53](https://github.com/puniyu/puniyu/pull/53)) - ([f55ab51](https://github.com/puniyu/puniyu/commit/f55ab519b9275c19773f16acfddff0c44db8e48c))
- 重构项目配置 - ([883dfa4](https://github.com/puniyu/puniyu/commit/883dfa4fb525d7e5c27821026d727e3d8eda8600))

### 📚 文档


- *(readme)* 添加社区QQ群链接 - ([60bf1e7](https://github.com/puniyu/puniyu/commit/60bf1e788718844583f6a2e6809b930b7d262775))
- Update README.md - ([52c8b9f](https://github.com/puniyu/puniyu/commit/52c8b9fcc37d46e92147e02fff0f2bf09b9eede2))

### ⚙️ 杂项


- *(cargo)* 更新配置以支持本地包路径依赖 - ([469cd82](https://github.com/puniyu/puniyu/commit/469cd8220e88d77898cc82944c91d75479c4665f))
- *(core)* 支持设置app_logo - ([3f4f713](https://github.com/puniyu/puniyu/commit/3f4f71344917f671468edfef639ec201440a1251))
- *(puniyu)* Release v0.7.7 - ([d44767b](https://github.com/puniyu/puniyu/commit/d44767bcb98388e8d88e3c163b6e344b986e59ee))
- *(workflow)* 修复发布ci - ([7cd84d6](https://github.com/puniyu/puniyu/commit/7cd84d6398285b00be4792942110421b71122cbe))
- *(workflow)* 引入稳定版构建配置 - ([fd81025](https://github.com/puniyu/puniyu/commit/fd81025eee0ef359320d1a8c482004772491ce0f))
- 初始化仓库 - ([51af186](https://github.com/puniyu/puniyu/commit/51af186b465079c274bb094dc3dc8b38ba959f15))

### Fest


- *(server)* 添加服务器路由支持 - ([7360323](https://github.com/puniyu/puniyu/commit/7360323b64400834013ad246d483dadf01db53ea))

## [0.7.8](https://github.com/puniyu/puniyu/compare/puniyu-v0.7.7...puniyu-v0.7.8) - 2026-01-12


### ⛰️ 新功能


- *(adapter)* 实现server适配器 ([#99](https://github.com/puniyu/puniyu/pull/99)) (由 @wuliya336 提供) (#99) - ([e1671ad](https://github.com/puniyu/puniyu/commit/e1671ad8ebc180f066be1ec58a508194d66e4850))
- *(config)* 添加全局命令前缀配置并优化模块结构 (由 @wuliya336 提供) - ([e0e5bb5](https://github.com/puniyu/puniyu/commit/e0e5bb5ac9eb24a37189a3d50b42d3db8db58dde))
- *(core)* 将 puniyu_logger 设置为可选依赖并更新版本获取逻辑 (由 @wuliya336 提供) - ([f49677a](https://github.com/puniyu/puniyu/commit/f49677ad1bcf2c5b64ff699241c427a7a7300cc0))
- *(plugin)* 新增服务端插件支持 (由 @wuliya336 提供) - ([7f15acf](https://github.com/puniyu/puniyu/commit/7f15acf148d002e33ef246b3a65a08866a44393f))
- *(plugin)* 添加基础插件及状态命令功能 (由 @wuliya336 提供) - ([2503696](https://github.com/puniyu/puniyu/commit/2503696fa293909244f5110e8504ae00bf8b962b))
- *(protocol)* 添加账户和适配器协议定义及Bot信息结构 ([#93](https://github.com/puniyu/puniyu/pull/93)) (由 @wuliya336 提供) (#93) - ([9611fba](https://github.com/puniyu/puniyu/commit/9611fba37182df82297bd1a4596473f9e81c2b5c))
- *(types)* 更新图片元素结构与消息处理逻辑 (由 @wuliya336 提供) - ([9b69689](https://github.com/puniyu/puniyu/commit/9b69689c679b3baa2a2d8acff99661b3e22f1766))

- Hook功能实现 ([#112](https://github.com/puniyu/puniyu/pull/112)) (由 @wuliya336 提供) (#112) - ([1e7bbf7](https://github.com/puniyu/puniyu/commit/1e7bbf7b6d25fcfb5c8fcedc6a68f29ff6b8c12e))


### 🐛 Bug 修复


- *(context)* 更新回复方法为泛型参数 (由 @wuliya336 提供) - ([fc41520](https://github.com/puniyu/puniyu/commit/fc41520a57b357ba1516cae99490a082afbbd9bb))



### 🚜 重构


- *(account)* 重构 AccountInfo 的结构体 (由 @wuliya336 提供) - ([0ae3ee2](https://github.com/puniyu/puniyu/commit/0ae3ee2ff7a242a6402458a124604676a191c2a8))
- *(adapter)* 重构适配器模块结构和依赖关系 (由 @wuliya336 提供) - ([3c8e01f](https://github.com/puniyu/puniyu/commit/3c8e01f3bc7046220d06e4fe9c0a919a4f9d26f2))
- *(adapter)* 重命名控制台适配器模块路径 (由 @wuliya336 提供) - ([5f9dcf3](https://github.com/puniyu/puniyu/commit/5f9dcf3c448225e54f1b3349b4746fb86fdf9897))
- *(app)* 重构应用构建器和适配器加载逻辑 (由 @wuliya336 提供) - ([58ddb3f](https://github.com/puniyu/puniyu/commit/58ddb3f0decf9b50b2e1270a8da6dc914a88dfd0))
- *(core)* 重新组织模块导入顺序并清理无用导出 (由 @wuliya336 提供) - ([8da3556](https://github.com/puniyu/puniyu/commit/8da35564bb6e99777d30d5c48497e11358c51915))
- *(core)* 移动logo资源到core包并优化构建配置 (由 @wuliya336 提供) - ([4e2d787](https://github.com/puniyu/puniyu/commit/4e2d787dd92318b45d1128d11ffc26c2451729c3))
- *(macros)* 重构proc-macro宏 ([#119](https://github.com/puniyu/puniyu/pull/119)) (由 @wuliya336 提供) (#119) - ([9942a1b](https://github.com/puniyu/puniyu/commit/9942a1baa348377af978d9224510316a0595a5f3))
- *(project)* 重构项目结构 (由 @wuliya336 提供) - ([b1b389f](https://github.com/puniyu/puniyu/commit/b1b389f25dae5899b49133be88ba348930117972))
- *(workspace)* 重构项目结构和依赖管理 (由 @wuliya336 提供) - ([520087e](https://github.com/puniyu/puniyu/commit/520087e38b49ff9e3aabe04be946038b81114b4d))

- 重构工作区crates ([#53](https://github.com/puniyu/puniyu/pull/53)) (由 @wuliya336 提供) (#53) - ([f55ab51](https://github.com/puniyu/puniyu/commit/f55ab519b9275c19773f16acfddff0c44db8e48c))
- 重构项目配置 (由 @wuliya336 提供) - ([883dfa4](https://github.com/puniyu/puniyu/commit/883dfa4fb525d7e5c27821026d727e3d8eda8600))


### 📚 文档


- *(readme)* 添加社区QQ群链接 (由 @wuliya336 提供) - ([60bf1e7](https://github.com/puniyu/puniyu/commit/60bf1e788718844583f6a2e6809b930b7d262775))

- Update README.md (由 @allcontributors[bot] 提供) (#17) - ([52c8b9f](https://github.com/puniyu/puniyu/commit/52c8b9fcc37d46e92147e02fff0f2bf09b9eede2))


### ⚙️ 杂项


- *(cargo)* 更新配置以支持本地包路径依赖 (由 @wuliya336 提供) - ([469cd82](https://github.com/puniyu/puniyu/commit/469cd8220e88d77898cc82944c91d75479c4665f))
- *(core)* 支持设置app_logo (由 @wuliya336 提供) - ([3f4f713](https://github.com/puniyu/puniyu/commit/3f4f71344917f671468edfef639ec201440a1251))
- *(puniyu)* Release v0.7.7 (由 @puniyu[bot] 提供) (#126) - ([d44767b](https://github.com/puniyu/puniyu/commit/d44767bcb98388e8d88e3c163b6e344b986e59ee))
- *(workflow)* 修复发布ci (由 @wuliya336 提供) - ([7cd84d6](https://github.com/puniyu/puniyu/commit/7cd84d6398285b00be4792942110421b71122cbe))
- *(workflow)* 引入稳定版构建配置 (由 @wuliya336 提供) - ([fd81025](https://github.com/puniyu/puniyu/commit/fd81025eee0ef359320d1a8c482004772491ce0f))

- 初始化仓库 (由 @wuliya336 提供) - ([51af186](https://github.com/puniyu/puniyu/commit/51af186b465079c274bb094dc3dc8b38ba959f15))


### Fest


- *(server)* 添加服务器路由支持 (由 @wuliya336 提供) - ([7360323](https://github.com/puniyu/puniyu/commit/7360323b64400834013ad246d483dadf01db53ea))




### 贡献者

* @puniyu[bot]
* @wuliya336
* @allcontributors[bot]
## [0.7.7](https://github.com/puniyu/puniyu/compare/puniyu-v0.7.6...puniyu-v0.7.7) - 2026-01-11


### ⛰️ 新功能


- *(adapter)* 实现server适配器 ([#99](https://github.com/puniyu/puniyu/pull/99)) (由 @wuliya336 提供) (#99) - ([e1671ad](https://github.com/puniyu/puniyu/commit/e1671ad8ebc180f066be1ec58a508194d66e4850))
- *(config)* 添加全局命令前缀配置并优化模块结构 (由 @wuliya336 提供) - ([e0e5bb5](https://github.com/puniyu/puniyu/commit/e0e5bb5ac9eb24a37189a3d50b42d3db8db58dde))
- *(core)* 将 puniyu_logger 设置为可选依赖并更新版本获取逻辑 (由 @wuliya336 提供) - ([f49677a](https://github.com/puniyu/puniyu/commit/f49677ad1bcf2c5b64ff699241c427a7a7300cc0))
- *(plugin)* 新增服务端插件支持 (由 @wuliya336 提供) - ([7f15acf](https://github.com/puniyu/puniyu/commit/7f15acf148d002e33ef246b3a65a08866a44393f))
- *(plugin)* 添加基础插件及状态命令功能 (由 @wuliya336 提供) - ([2503696](https://github.com/puniyu/puniyu/commit/2503696fa293909244f5110e8504ae00bf8b962b))
- *(protocol)* 添加账户和适配器协议定义及Bot信息结构 ([#93](https://github.com/puniyu/puniyu/pull/93)) (由 @wuliya336 提供) (#93) - ([9611fba](https://github.com/puniyu/puniyu/commit/9611fba37182df82297bd1a4596473f9e81c2b5c))
- *(types)* 更新图片元素结构与消息处理逻辑 (由 @wuliya336 提供) - ([9b69689](https://github.com/puniyu/puniyu/commit/9b69689c679b3baa2a2d8acff99661b3e22f1766))

- Hook功能实现 ([#112](https://github.com/puniyu/puniyu/pull/112)) (由 @wuliya336 提供) (#112) - ([1e7bbf7](https://github.com/puniyu/puniyu/commit/1e7bbf7b6d25fcfb5c8fcedc6a68f29ff6b8c12e))


### 🐛 Bug 修复


- *(context)* 更新回复方法为泛型参数 (由 @wuliya336 提供) - ([fc41520](https://github.com/puniyu/puniyu/commit/fc41520a57b357ba1516cae99490a082afbbd9bb))



### 🚜 重构


- *(account)* 重构 AccountInfo 的结构体 (由 @wuliya336 提供) - ([0ae3ee2](https://github.com/puniyu/puniyu/commit/0ae3ee2ff7a242a6402458a124604676a191c2a8))
- *(adapter)* 重构适配器模块结构和依赖关系 (由 @wuliya336 提供) - ([3c8e01f](https://github.com/puniyu/puniyu/commit/3c8e01f3bc7046220d06e4fe9c0a919a4f9d26f2))
- *(adapter)* 重命名控制台适配器模块路径 (由 @wuliya336 提供) - ([5f9dcf3](https://github.com/puniyu/puniyu/commit/5f9dcf3c448225e54f1b3349b4746fb86fdf9897))
- *(app)* 重构应用构建器和适配器加载逻辑 (由 @wuliya336 提供) - ([58ddb3f](https://github.com/puniyu/puniyu/commit/58ddb3f0decf9b50b2e1270a8da6dc914a88dfd0))
- *(core)* 重新组织模块导入顺序并清理无用导出 (由 @wuliya336 提供) - ([8da3556](https://github.com/puniyu/puniyu/commit/8da35564bb6e99777d30d5c48497e11358c51915))
- *(core)* 移动logo资源到core包并优化构建配置 (由 @wuliya336 提供) - ([4e2d787](https://github.com/puniyu/puniyu/commit/4e2d787dd92318b45d1128d11ffc26c2451729c3))
- *(macros)* 重构proc-macro宏 ([#119](https://github.com/puniyu/puniyu/pull/119)) (由 @wuliya336 提供) (#119) - ([9942a1b](https://github.com/puniyu/puniyu/commit/9942a1baa348377af978d9224510316a0595a5f3))
- *(project)* 重构项目结构 (由 @wuliya336 提供) - ([b1b389f](https://github.com/puniyu/puniyu/commit/b1b389f25dae5899b49133be88ba348930117972))
- *(workspace)* 重构项目结构和依赖管理 (由 @wuliya336 提供) - ([520087e](https://github.com/puniyu/puniyu/commit/520087e38b49ff9e3aabe04be946038b81114b4d))

- 重构工作区crates ([#53](https://github.com/puniyu/puniyu/pull/53)) (由 @wuliya336 提供) (#53) - ([f55ab51](https://github.com/puniyu/puniyu/commit/f55ab519b9275c19773f16acfddff0c44db8e48c))
- 重构项目配置 (由 @wuliya336 提供) - ([883dfa4](https://github.com/puniyu/puniyu/commit/883dfa4fb525d7e5c27821026d727e3d8eda8600))


### 📚 文档


- *(readme)* 添加社区QQ群链接 (由 @wuliya336 提供) - ([60bf1e7](https://github.com/puniyu/puniyu/commit/60bf1e788718844583f6a2e6809b930b7d262775))

- Update README.md (由 @allcontributors[bot] 提供) (#17) - ([52c8b9f](https://github.com/puniyu/puniyu/commit/52c8b9fcc37d46e92147e02fff0f2bf09b9eede2))


### ⚙️ 杂项


- *(cargo)* 更新配置以支持本地包路径依赖 (由 @wuliya336 提供) - ([469cd82](https://github.com/puniyu/puniyu/commit/469cd8220e88d77898cc82944c91d75479c4665f))
- *(core)* 支持设置app_logo (由 @wuliya336 提供) - ([3f4f713](https://github.com/puniyu/puniyu/commit/3f4f71344917f671468edfef639ec201440a1251))
- *(workflow)* 修复发布ci (由 @wuliya336 提供) - ([7cd84d6](https://github.com/puniyu/puniyu/commit/7cd84d6398285b00be4792942110421b71122cbe))
- *(workflow)* 引入稳定版构建配置 (由 @wuliya336 提供) - ([fd81025](https://github.com/puniyu/puniyu/commit/fd81025eee0ef359320d1a8c482004772491ce0f))

- 初始化仓库 (由 @wuliya336 提供) - ([51af186](https://github.com/puniyu/puniyu/commit/51af186b465079c274bb094dc3dc8b38ba959f15))


### Fest


- *(server)* 添加服务器路由支持 (由 @wuliya336 提供) - ([7360323](https://github.com/puniyu/puniyu/commit/7360323b64400834013ad246d483dadf01db53ea))




### 贡献者

* @puniyu[bot]
* @wuliya336
* @allcontributors[bot]
## [0.7.6](https://github.com/puniyu/puniyu/compare/puniyu-v0.7.3...puniyu-v0.7.6) - 2026-01-11


### ⛰️ 新功能


- *(adapter)* 实现server适配器 ([#99](https://github.com/puniyu/puniyu/pull/99)) (由 @wuliya336 提供) (#99) - ([e1671ad](https://github.com/puniyu/puniyu/commit/e1671ad8ebc180f066be1ec58a508194d66e4850))
- *(config)* 添加全局命令前缀配置并优化模块结构 (由 @wuliya336 提供) - ([e0e5bb5](https://github.com/puniyu/puniyu/commit/e0e5bb5ac9eb24a37189a3d50b42d3db8db58dde))
- *(core)* 将 puniyu_logger 设置为可选依赖并更新版本获取逻辑 (由 @wuliya336 提供) - ([f49677a](https://github.com/puniyu/puniyu/commit/f49677ad1bcf2c5b64ff699241c427a7a7300cc0))
- *(plugin)* 新增服务端插件支持 (由 @wuliya336 提供) - ([7f15acf](https://github.com/puniyu/puniyu/commit/7f15acf148d002e33ef246b3a65a08866a44393f))
- *(plugin)* 添加基础插件及状态命令功能 (由 @wuliya336 提供) - ([2503696](https://github.com/puniyu/puniyu/commit/2503696fa293909244f5110e8504ae00bf8b962b))
- *(protocol)* 添加账户和适配器协议定义及Bot信息结构 ([#93](https://github.com/puniyu/puniyu/pull/93)) (由 @wuliya336 提供) (#93) - ([9611fba](https://github.com/puniyu/puniyu/commit/9611fba37182df82297bd1a4596473f9e81c2b5c))
- *(types)* 更新图片元素结构与消息处理逻辑 (由 @wuliya336 提供) - ([9b69689](https://github.com/puniyu/puniyu/commit/9b69689c679b3baa2a2d8acff99661b3e22f1766))

- Hook功能实现 ([#112](https://github.com/puniyu/puniyu/pull/112)) (由 @wuliya336 提供) (#112) - ([1e7bbf7](https://github.com/puniyu/puniyu/commit/1e7bbf7b6d25fcfb5c8fcedc6a68f29ff6b8c12e))


### 🐛 Bug 修复


- *(context)* 更新回复方法为泛型参数 (由 @wuliya336 提供) - ([fc41520](https://github.com/puniyu/puniyu/commit/fc41520a57b357ba1516cae99490a082afbbd9bb))



### 🚜 重构


- *(account)* 重构 AccountInfo 的结构体 (由 @wuliya336 提供) - ([0ae3ee2](https://github.com/puniyu/puniyu/commit/0ae3ee2ff7a242a6402458a124604676a191c2a8))
- *(adapter)* 重构适配器模块结构和依赖关系 (由 @wuliya336 提供) - ([3c8e01f](https://github.com/puniyu/puniyu/commit/3c8e01f3bc7046220d06e4fe9c0a919a4f9d26f2))
- *(adapter)* 重命名控制台适配器模块路径 (由 @wuliya336 提供) - ([5f9dcf3](https://github.com/puniyu/puniyu/commit/5f9dcf3c448225e54f1b3349b4746fb86fdf9897))
- *(app)* 重构应用构建器和适配器加载逻辑 (由 @wuliya336 提供) - ([58ddb3f](https://github.com/puniyu/puniyu/commit/58ddb3f0decf9b50b2e1270a8da6dc914a88dfd0))
- *(core)* 重新组织模块导入顺序并清理无用导出 (由 @wuliya336 提供) - ([8da3556](https://github.com/puniyu/puniyu/commit/8da35564bb6e99777d30d5c48497e11358c51915))
- *(core)* 移动logo资源到core包并优化构建配置 (由 @wuliya336 提供) - ([4e2d787](https://github.com/puniyu/puniyu/commit/4e2d787dd92318b45d1128d11ffc26c2451729c3))
- *(macros)* 重构proc-macro宏 ([#119](https://github.com/puniyu/puniyu/pull/119)) (由 @wuliya336 提供) (#119) - ([9942a1b](https://github.com/puniyu/puniyu/commit/9942a1baa348377af978d9224510316a0595a5f3))
- *(project)* 重构项目结构 (由 @wuliya336 提供) - ([b1b389f](https://github.com/puniyu/puniyu/commit/b1b389f25dae5899b49133be88ba348930117972))
- *(workspace)* 重构项目结构和依赖管理 (由 @wuliya336 提供) - ([520087e](https://github.com/puniyu/puniyu/commit/520087e38b49ff9e3aabe04be946038b81114b4d))

- 重构工作区crates ([#53](https://github.com/puniyu/puniyu/pull/53)) (由 @wuliya336 提供) (#53) - ([f55ab51](https://github.com/puniyu/puniyu/commit/f55ab519b9275c19773f16acfddff0c44db8e48c))
- 重构项目配置 (由 @wuliya336 提供) - ([883dfa4](https://github.com/puniyu/puniyu/commit/883dfa4fb525d7e5c27821026d727e3d8eda8600))


### 📚 文档


- *(readme)* 添加社区QQ群链接 (由 @wuliya336 提供) - ([60bf1e7](https://github.com/puniyu/puniyu/commit/60bf1e788718844583f6a2e6809b930b7d262775))

- Update README.md (由 @allcontributors[bot] 提供) (#17) - ([52c8b9f](https://github.com/puniyu/puniyu/commit/52c8b9fcc37d46e92147e02fff0f2bf09b9eede2))


### ⚙️ 杂项


- *(cargo)* 更新配置以支持本地包路径依赖 (由 @wuliya336 提供) - ([469cd82](https://github.com/puniyu/puniyu/commit/469cd8220e88d77898cc82944c91d75479c4665f))
- *(core)* 支持设置app_logo (由 @wuliya336 提供) - ([3f4f713](https://github.com/puniyu/puniyu/commit/3f4f71344917f671468edfef639ec201440a1251))
- *(workflow)* 修复发布ci (由 @wuliya336 提供) - ([7cd84d6](https://github.com/puniyu/puniyu/commit/7cd84d6398285b00be4792942110421b71122cbe))
- *(workflow)* 引入稳定版构建配置 (由 @wuliya336 提供) - ([fd81025](https://github.com/puniyu/puniyu/commit/fd81025eee0ef359320d1a8c482004772491ce0f))

- 初始化仓库 (由 @wuliya336 提供) - ([51af186](https://github.com/puniyu/puniyu/commit/51af186b465079c274bb094dc3dc8b38ba959f15))


### Fest


- *(server)* 添加服务器路由支持 (由 @wuliya336 提供) - ([7360323](https://github.com/puniyu/puniyu/commit/7360323b64400834013ad246d483dadf01db53ea))




### 贡献者

* @wuliya336
* @puniyu[bot]
* @allcontributors[bot]
