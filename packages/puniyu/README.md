# puniyu

`puniyu` 是工作区里的可执行入口，用来把核心框架、命令处理器、控制台适配器和基础插件组装成一个可以直接运行的应用。

## 定位

如果把整个仓库看作一套积木，那么 `packages/puniyu` 就是把这些积木真正拼起来的地方。它不是底层抽象库，而是默认启动方式的参考实现。

## 默认装配内容

当前入口会通过 `App::builder()` 依次装配：

- `puniyu_core`
- `puniyu_handler_command`
- `puniyu_adapter_console`
- `puniyu_plugin_basic`

这意味着它既可以作为本地运行入口，也可以作为理解框架装配方式的最短路径。

## 如何启动

```bash
cargo run -p puniyu
```

启动后可以直接在控制台输入消息内容，默认按好友消息处理；也可以用下面的形式模拟不同来源：

```text
group:text:你好，群消息
friend:at:10001
grouptemp:text:临时会话消息
```

## 何时阅读这个模块

- 你想快速跑起一个最小可用的 Puniyu 实例
- 你想知道工作区默认是如何组装应用的
- 你准备做自己的可执行入口，想找一个基础模板

## 相关模块

- `puniyu_core`
- `puniyu_handler_command`
- `puniyu_adapter_console`
- `puniyu_plugin_basic`

> [!NOTE]
> 这个包更偏向“默认入口”和“装配示例”，真正的通用能力主要分布在 `crates/` 下的各个基础库中。
