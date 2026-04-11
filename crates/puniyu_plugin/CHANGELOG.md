# 变更日志

## [0.8.0]

### ⛰️ 新功能


- *(core)* 添加配置注册功能并优化初始化逻辑 - ([5593859](https://github.com/puniyu/puniyu/commit/5593859d698f03f01b5e3dd1685984bdfe14c96c))
- 添加命令解析器和配置管理功能 - ([075d485](https://github.com/puniyu/puniyu/commit/075d48589a9da7ec45fbd88c60cbe5a3435a3163))

### 🐛 Bug 修复


- *(command)* 添加命令返回类型验证并优化消息事件处理 - ([8db2173](https://github.com/puniyu/puniyu/commit/8db21733d5fe99034f285bb4035e4dbf30340eee))
- *(plugin)* 修复插件宏中空函数体的返回类型验证 - ([da7f237](https://github.com/puniyu/puniyu/commit/da7f2379b0f7860c04cc0e28df423e534b48b60b))
- 修复文档测试 - ([ed38db1](https://github.com/puniyu/puniyu/commit/ed38db1ff6a8dafb599bd911f5363ac6d3d35bce))
- 修正 command_id 初始化索引 - ([7a9bac2](https://github.com/puniyu/puniyu/commit/7a9bac230fa24b936e3130852bb12a4cdbd1be55))

### 🚜 重构


- *(adapter)* 重构适配器模块结构和依赖关系 - ([3c8e01f](https://github.com/puniyu/puniyu/commit/3c8e01f3bc7046220d06e4fe9c0a919a4f9d26f2))
- *(adapter)* 统一HandlerResult类型并优化API实现 - ([3926f29](https://github.com/puniyu/puniyu/commit/3926f29742bb000b8b861c35205257b85be9017e))
- *(adapter)* 重构适配器注册相关内容 - ([cdec0b9](https://github.com/puniyu/puniyu/commit/cdec0b9e002e12aa6effb6dfda8dc3b331a1fec1))
- *(contact)* 重构联系人类型系统，引入 trait 抽象 - ([c403f08](https://github.com/puniyu/puniyu/commit/c403f08b8aaebfa071995c8179c9d75a8ae532d9))
- *(core)* 重构整个框架的核心实现 ([#165](https://github.com/puniyu/puniyu/pull/165)) - ([9482b34](https://github.com/puniyu/puniyu/commit/9482b34a76fecf05b3475aecf01df93d9e8994ff))
- *(core)* 重构部分核心实现 ([#150](https://github.com/puniyu/puniyu/pull/150)) - ([e06459e](https://github.com/puniyu/puniyu/commit/e06459e40f1ec37f8e0de7427e6905c9bb295d5e))
- *(core)* 重新组织模块导入顺序并清理无用导出 - ([8da3556](https://github.com/puniyu/puniyu/commit/8da35564bb6e99777d30d5c48497e11358c51915))
- *(plugin)* 统一插件初始化返回类型为HandlerResult - ([31c1e58](https://github.com/puniyu/puniyu/commit/31c1e584885121f6464e761d7008ce1548460a57))
- *(puniyu_logger)* 更新日志记录, 多个包模块更新 - ([b551405](https://github.com/puniyu/puniyu/commit/b55140558783461e03da9a27aae099f97d46a4a6))
- *(registry)* 重构注册表 - ([4e93d6c](https://github.com/puniyu/puniyu/commit/4e93d6c44c888309cd236a6d92a38e543e943e2f))
- 重构事件/适配器api/上下文 ([#178](https://github.com/puniyu/puniyu/pull/178)) - ([fe74041](https://github.com/puniyu/puniyu/commit/fe74041525fac5102ed96ed2a7389ecd1a27f7ed))
- 重构项目配置 - ([883dfa4](https://github.com/puniyu/puniyu/commit/883dfa4fb525d7e5c27821026d727e3d8eda8600))
- 优化全局注册表, 减少调用时开销 ([#144](https://github.com/puniyu/puniyu/pull/144)) - ([1447162](https://github.com/puniyu/puniyu/commit/1447162841cbebfba06e12eaad9fea263aa0436f))
- 使用jiff替换chrono库并优化时间处理 ([#179](https://github.com/puniyu/puniyu/pull/179)) - ([8d0257a](https://github.com/puniyu/puniyu/commit/8d0257a9663c43dceafa6e35f7e6f382425df727))
- 重构工作区crates ([#53](https://github.com/puniyu/puniyu/pull/53)) - ([f55ab51](https://github.com/puniyu/puniyu/commit/f55ab519b9275c19773f16acfddff0c44db8e48c))

### 📚 文档


- *(puniyu_macros)* 更新文档并完善API注释 - ([970a92a](https://github.com/puniyu/puniyu/commit/970a92a87bf99e53c8156c3f7d3a30b3b3c6425a))
- *(readme)* 添加社区QQ群链接 - ([60bf1e7](https://github.com/puniyu/puniyu/commit/60bf1e788718844583f6a2e6809b930b7d262775))
- Update README.md - ([52c8b9f](https://github.com/puniyu/puniyu/commit/52c8b9fcc37d46e92147e02fff0f2bf09b9eede2))

### ⚙️ 杂项


- 初始化仓库 - ([51af186](https://github.com/puniyu/puniyu/commit/51af186b465079c274bb094dc3dc8b38ba959f15))

