# puniyu_server

HTTP 服务能力库，同时提供可直接运行的服务入口。

## 特征

- 提供 `run_server`、`run_server_spawn`、`stop_server`、`restart_server`
- 基于 Actix Web 提供统一服务能力
- 支持独立启动 HTTP 服务
- 支持通过参数指定监听地址和端口

## 快速开始

```bash
cargo run -p puniyu_server
```

也可以显式指定监听地址和端口：

```bash
cargo run -p puniyu_server -- --host 127.0.0.1 --port 33700
```