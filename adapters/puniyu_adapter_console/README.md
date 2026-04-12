# puniyu_adapter_console

默认控制台适配器，把标准输入转换为 Puniyu 事件，方便本地开发和调试。

## 特征

- 从 stdin 读取输入并转换为事件
- 支持好友、群聊、群临时会话、频道（guild）消息
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
grouptemp:text:你好，群临时消息
guild:text:你好，频道消息
```

输入 `quit`、`exit` 或 `q` 可以退出。

## 输入格式

默认格式：

```text
<scene>:<element_type>:<content>
```

其中：

- `scene` 支持：`friend`、`group`、`grouptemp`、`guild`
- `element_type` 支持：`text`、`at`、`image`、`json`、`video`、`record`、`file`、`xml`

如果不写 `scene`，默认按 `friend` 处理：

```text
hello world
```

等价于：

```text
friend:text:hello world
```

示例：

```text
friend:text:你好
group:text:群消息
grouptemp:image:https://example.com/a.png
guild:text:频道消息
group:json:{"type":"test"}
```

## 模块结构

当前实现已拆成以下职责：

- `src/lib.rs`：初始化 adapter / bot、读取 stdin、驱动输入循环
- `src/input.rs`：解析控制台输入为中间结构
- `src/common/event_factory.rs`：把输入转换为消息事件并派发
- `src/common/ids.rs`：生成随机事件 ID / 消息 ID

其中 guild 当前采用简洁模式，支持：

```text
guild:text:内容
```

内部会使用默认测试标识构造 guild contact / sender / event，用于本地调试频道消息链路。
