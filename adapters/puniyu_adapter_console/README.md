# puniyu_adapter_console

控制台适配器，把标准输入内容转换为 Puniyu 事件，方便本地开发、调试和示例演示。

## 定位

`puniyu_adapter_console` 是 Puniyu 工作区中的一个 adapter。

## 提供内容

- 注册一个名为 `console` 的 bot。
- 从 stdin 读取输入并转换为好友、群聊、群临时会话消息。
- 支持 `text`、`at`、`image`、`json`、`video`、`record`、`file`、`xml` 等基础元素输入格式。

## 何时使用

当你需要在没有真实平台接入的情况下验证命令、事件流和插件逻辑。

## 输入示例

```text
hello world
group:text:你好，群消息
friend:at:10001
group:image:https://example.com/demo.png
```

输入 `quit`、`exit` 或 `q` 可以退出。

## 相关模块

- `puniyu_adapter`
- `puniyu_runtime`
- `puniyu_event`
- `puniyu_core`

> [!NOTE]
> README 以当前工作区代码结构为准，适合快速了解模块职责；更细的 API 细节请直接阅读源码。
