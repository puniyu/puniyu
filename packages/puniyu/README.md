# puniyu

默认可执行入口，用来把核心框架、命令处理器、控制台适配器和基础插件组装成一个可以直接运行的应用。

## 特征

- 基于 `puniyu_core::App::builder()` 装配应用
- 默认集成 `puniyu_handler_command`
- 默认集成 `puniyu_adapter_console`
- 默认集成 `puniyu_plugin_basic`

## 快速开始

```bash
cargo run -p puniyu
```

启动后可以直接在控制台输入消息，默认按好友消息处理，也可以用下面的形式模拟不同来源：

```text
group:text:你好，群消息
friend:at:10001
grouptemp:text:临时会话消息
```