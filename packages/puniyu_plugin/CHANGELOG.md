# 变更日志
## [0.7.5](https://github.com/puniyu/puniyu/compare/puniyu_plugin-v0.7.4...puniyu_plugin-v0.7.5) - 2026-01-10


### ⛰️ 新功能



- Hook功能实现 ([#112](https://github.com/puniyu/puniyu/pull/112)) (由 @wuliya336 提供) (#112) - ([1e7bbf7](https://github.com/puniyu/puniyu/commit/1e7bbf7b6d25fcfb5c8fcedc6a68f29ff6b8c12e))


### 🚜 重构


- *(macros)* 重构proc-macro宏 ([#119](https://github.com/puniyu/puniyu/pull/119)) (由 @wuliya336 提供) (#119) - ([9942a1b](https://github.com/puniyu/puniyu/commit/9942a1baa348377af978d9224510316a0595a5f3))




### 贡献者

* @wuliya336
## [0.7.4](https://github.com/puniyu/puniyu/compare/puniyu_plugin-v0.7.3...puniyu_plugin-v0.7.4) - 2026-01-07


### ⛰️ 新功能


- *(command)* 添加命令权限控制功能 (由 @shiwuliya 提供) - ([cc0013a](https://github.com/puniyu/puniyu/commit/cc0013aff04d8efea0b9cdda3f11eae4d1eac97b))
- *(config)* 将配置序列化格式从 JSON 切换为 TOML (由 @shiwuliya 提供) - ([48fc976](https://github.com/puniyu/puniyu/commit/48fc976274386311b62fa1d344807a68432b99f0))
- *(macro)* 重构宏系统并增强命令参数支持 (由 @shiwuliya 提供) - ([e8f3c23](https://github.com/puniyu/puniyu/commit/e8f3c23e42cd94fb567dce569cbe4477014300b1))
- *(plugin)* 新增服务端插件支持 (由 @shiwuliya 提供) - ([7f15acf](https://github.com/puniyu/puniyu/commit/7f15acf148d002e33ef246b3a65a08866a44393f))
- *(plugin)* 添加基础插件及状态命令功能 (由 @shiwuliya 提供) - ([2503696](https://github.com/puniyu/puniyu/commit/2503696fa293909244f5110e8504ae00bf8b962b))
- *(plugin)* 支持命令参数的位置和命名模式 (由 @shiwuliya 提供) - ([85e92d4](https://github.com/puniyu/puniyu/commit/85e92d4ec50367ad3d1e1194ee1542ce74dd82dd))
- *(plugin)* 添加插件配置支持 (由 @shiwuliya 提供) - ([dc7d1eb](https://github.com/puniyu/puniyu/commit/dc7d1ebcf2245f53f3a58b203edd405aa7cc8c1c))



### 🐛 Bug 修复


- *(plugin)* 修复缺少的导入 (由 @shiwuliya 提供) - ([69e01e4](https://github.com/puniyu/puniyu/commit/69e01e4d80ae0e0c47e2d7e2d27ce24de70ae227))
- *(puniyu_plugin)* 添加缺少的导入 (由 @shiwuliya 提供) - ([23f7f8a](https://github.com/puniyu/puniyu/commit/23f7f8a459f941971a203063d6215c9779b74411))



### 🚜 重构


- *(adapter)* 重构适配器模块结构和依赖关系 (由 @shiwuliya 提供) - ([3c8e01f](https://github.com/puniyu/puniyu/commit/3c8e01f3bc7046220d06e4fe9c0a919a4f9d26f2))
- *(adapter)* 重构适配器API结构并添加模块化接口 ([#100](https://github.com/puniyu/puniyu/pull/100)) (由 @shiwuliya 提供) (#100) - ([6558f4f](https://github.com/puniyu/puniyu/commit/6558f4faed67a89a59c4c2790a7ba986581888a5))
- *(adapter)* 独立account模块 (由 @shiwuliya 提供) - ([0f4c175](https://github.com/puniyu/puniyu/commit/0f4c1758ecc817c2048fec38265edbf630252cb0))
- *(command)* 重构命令处理系统 (由 @shiwuliya 提供) (#96) - ([549db91](https://github.com/puniyu/puniyu/commit/549db91d4bd063616c85bced71f3ea69431bf6c0))
- *(command)* 重构命令处理结果类型和参数验证 (由 @shiwuliya 提供) - ([58d4eeb](https://github.com/puniyu/puniyu/commit/58d4eebb41cacabc7663b40a93181b789feb1e0a))
- *(core)* 重构元素宏和消息构建器实现 (由 @shiwuliya 提供) - ([abdd4ac](https://github.com/puniyu/puniyu/commit/abdd4ac369560284bc77420d25f99035d3adb4a7))
- *(core)* 重新组织模块导入顺序并清理无用导出 (由 @shiwuliya 提供) - ([8da3556](https://github.com/puniyu/puniyu/commit/8da35564bb6e99777d30d5c48497e11358c51915))
- *(element)* 重构消息元素模块结构 (由 @shiwuliya 提供) - ([2aca906](https://github.com/puniyu/puniyu/commit/2aca906d9f9d44e77753e3784539be24a16f878d))
- *(event)* 重构事件系统并重命名事件总线为事件模块 (由 @shiwuliya 提供) (#98) - ([39093d4](https://github.com/puniyu/puniyu/commit/39093d4da202aaac142cd134d38e55e9a40b526d))
- *(project)* 重构项目结构 (由 @shiwuliya 提供) - ([b1b389f](https://github.com/puniyu/puniyu/commit/b1b389f25dae5899b49133be88ba348930117972))

- 重构工作区crates ([#53](https://github.com/puniyu/puniyu/pull/53)) (由 @shiwuliya 提供) (#53) - ([f55ab51](https://github.com/puniyu/puniyu/commit/f55ab519b9275c19773f16acfddff0c44db8e48c))


### 🎨 样式


- *(code)* 项目格式化 (由 @shiwuliya 提供) - ([dc3d850](https://github.com/puniyu/puniyu/commit/dc3d850bcca149de821bc5ec700ae98f567f4a79))



### ⚙️ 杂项


- *(version)* 导出version类型 (由 @shiwuliya 提供) - ([7d8d041](https://github.com/puniyu/puniyu/commit/7d8d0410618a915a7084caa2cfb8bcb29c0754d2))
- *(workflow)* 修复发布ci (由 @shiwuliya 提供) - ([7cd84d6](https://github.com/puniyu/puniyu/commit/7cd84d6398285b00be4792942110421b71122cbe))



### Fest


- *(server)* 添加服务器路由支持 (由 @shiwuliya 提供) - ([7360323](https://github.com/puniyu/puniyu/commit/7360323b64400834013ad246d483dadf01db53ea))




### 贡献者

* @shiwuliya
* @puniyu[bot]
