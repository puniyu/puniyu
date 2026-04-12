# puniyu_adapter_console

默认控制台适配器，把标准输入转换为 Puniyu 事件，方便本地开发和调试。

## 特征

- 从 stdin 读取输入并转换为事件
- 支持好友、群聊、群临时会话消息
- 支持 `text`、`at`、`image`、`json`、`video`、`record`、`file`、`xml` 等输入格式
- 可用于验证命令、事件流和插件逻辑

## 快速开始

先运行默认入口：

```bash
cargo run -p puniyu
```

然后在控制台输入：

```text
hello world
group:text:你好，群消息
friend:at:10001
```

输入 `quit`、`exit` 或 `q` 可以退出。