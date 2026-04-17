# puniyu IPC 协议定稿（极简版）

## 目标

适用于：

- 宿主 ↔ 插件双向调用
- 动态服务派发
- 少字段、少模型、少约束
- 支持结构化数据和二进制数据

---

## 1. 帧格式

```text
| u32 frame_len | u8 version | u8 kind | payload(msgpack) |
```

### 字段说明

#### `frame_len: u32`
后续所有字节长度，即：

```text
version + kind + payload
```

建议使用：

```text
big-endian
```

#### `version: u8`
协议版本号。

当前固定为：

```text
1
```

#### `kind: u8`
消息类型：

```text
0 = request
1 = response
2 = error
3 = cancel
4 = event
```

#### `payload`
使用 **MessagePack** 编码。

---

## 2. 消息结构

### 2.1 Request

```json
{
  "id": 123,
  "service": "plugin.command.ping",
  "params": ["a", "b"]
}
```

#### 字段
- `id`
  - 请求唯一标识
  - 用于 request / response / error / cancel 配对
- `service`
  - 完整调用目标
- `params`
  - 调用参数
  - 为任意 MessagePack 值

---

### 2.2 Response

```json
{
  "id": 123,
  "result": "pong"
}
```

#### 字段
- `id`
  - 对应请求 ID
- `result`
  - 返回结果
  - 为任意 MessagePack 值

---

### 2.3 Error

```json
{
  "id": 123,
  "error": {
    "code": "METHOD_NOT_FOUND",
    "message": "service not found"
  }
}
```

#### 字段
- `id`
  - 对应请求 ID
- `error.code`
  - 错误码
- `error.message`
  - 错误描述

---

### 2.4 Cancel

```json
{
  "id": 123
}
```

#### 语义
取消某个正在处理的请求。

说明：

- 是否真正停止执行，由被调用方决定
- 调用方本地可以立即结束等待

---

### 2.5 Event（可选）

```json
{
  "service": "host.event.shutdown",
  "params": null
}
```

说明：

- Event 不要求 `id`
- 用于单向通知或广播

---

## 3. 关键字段约束

### `service`
`service` 直接表示调用目标，不再拆 `method`。

建议命名风格：

```text
plugin.command.ping
plugin.task.cleanup
plugin.lifecycle.init
host.command.register
host.plugin.load
```

建议约束：

- 全小写
- 用 `.` 分层
- 语义稳定
- 可支持前缀匹配

---

### `id`
建议：

- 单连接内唯一
- 类型使用 `u64`
- 本地自增生成

当前语义：

- 请求配对 ID
- 当前阶段也可兼作 trace 标识

---

### `params`
定义为：

> 任意合法的 MessagePack 值

所以它可以是：

- `nil`
- `bool`
- `int`
- `float`
- `str`
- `array`
- `map`
- `bin(bytes)`

#### 示例

##### 无参数
```json
{
  "id": 1,
  "service": "plugin.health",
  "params": null
}
```

##### 数组参数
```json
{
  "id": 2,
  "service": "plugin.command.ping",
  "params": ["a", "b"]
}
```

##### 对象参数
```json
{
  "id": 3,
  "service": "plugin.task.schedule",
  "params": {
    "name": "daily_job",
    "cron": "0 0 * * *"
  }
}
```

##### 直接传二进制
```json
{
  "id": 4,
  "service": "plugin.stream.chunk",
  "params": "<msgpack bin>"
}
```

##### 对象中带二进制
```json
{
  "id": 5,
  "service": "plugin.file.write",
  "params": {
    "path": "a.txt",
    "data": "<msgpack bin>"
  }
}
```

---

### `result`
同样定义为任意 MessagePack 值。

例如：

- `null`
- 字符串
- 数组
- 对象
- bytes

---

## 4. 错误码建议

先统一用字符串即可：

```text
BAD_REQUEST
METHOD_NOT_FOUND
INVALID_PARAMS
TIMEOUT
CANCELLED
INTERNAL_ERROR
NOT_SUPPORTED
UNAUTHORIZED
FORBIDDEN
```

示例：

```json
{
  "id": 10,
  "error": {
    "code": "INVALID_PARAMS",
    "message": "expected array params"
  }
}
```

---

## 5. 明确不放进协议的字段

当前版本不引入：

- `method`
- `command`
- `timeout_ms`
- `trace_id`
- `encoding`
- `body_type`

原因：

- `service` 已足够表达调用目标
- `params` 已直接承载数据
- `timeout` 是本地调用策略
- `id` 当前可兼作 trace 标识
- 不再混用 Protobuf，因此无需 `encoding/body_type`

---

## 6. 调用流程

### 发请求
```json
{
  "id": 100,
  "service": "plugin.command.ping",
  "params": ["hello"]
}
```

### 成功响应
```json
{
  "id": 100,
  "result": "pong"
}
```

### 失败响应
```json
{
  "id": 100,
  "error": {
    "code": "INTERNAL_ERROR",
    "message": "xxx"
  }
}
```

### 取消请求
```json
{
  "id": 100
}
```

---

## 7. 建议的实现约束

### 最大帧长度
建议限制，例如：

```text
8MB
```

防止异常长度导致内存问题。

### 未知版本
收到未知 `version`：

- 直接断开
- 或返回 `NOT_SUPPORTED`

### 未知 kind
可直接视为协议错误。

### 未注册 service
返回：

```text
METHOD_NOT_FOUND
```

---

## 8. 最小可执行规范

如果压缩成最核心内容，就是：

### 帧
```text
| u32 frame_len | u8 version | u8 kind | msgpack payload |
```

### request
```json
{
  "id": 1,
  "service": "plugin.command.ping",
  "params": ["a", "b"]
}
```

### response
```json
{
  "id": 1,
  "result": "pong"
}
```

### error
```json
{
  "id": 1,
  "error": {
    "code": "METHOD_NOT_FOUND",
    "message": "service not found"
  }
}
```

### cancel
```json
{
  "id": 1
}
```

---

## 9. 一句话总结

最终协议就是三件事：

1. `service` 表示调用目标
2. `params/result` 是任意 MessagePack 值
3. `id` 用于请求配对，也可暂时兼作 trace 标识
