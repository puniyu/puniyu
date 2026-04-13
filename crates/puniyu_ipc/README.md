# puniyu_ipc

本地 IPC 库，提供基于 `interprocess` local socket 的 Tokio 连接能力，以及面向 `bytes::Bytes` 的定长帧收发接口。

## 特征

- 提供基于 namespaced local socket 的本地 IPC 监听与连接能力
- 提供 `Bytes` 二进制消息的长度前缀帧协议收发接口
- 内置 1 GiB 单帧大小限制，避免异常大消息导致内存分配失控
- 适合作为 puniyu 进程间通信的基础传输层

## 快速开始

可以先从服务端监听和客户端连接的最小示例开始：

```rust
use bytes::Bytes;

async fn server() -> std::io::Result<()> {
    let server = puniyu_ipc::Server::bind().await?;
    let mut stream = server.accept().await?;

    let request = puniyu_ipc::recv_bytes(&mut stream).await?;
    puniyu_ipc::send_bytes(&mut stream, request.as_ref()).await?;
    Ok(())
}

async fn client() -> std::io::Result<()> {
    let mut stream = puniyu_ipc::connect().await?;

    puniyu_ipc::send_bytes(&mut stream, b"ping").await?;
    let response = puniyu_ipc::recv_bytes(&mut stream).await?;

    assert_eq!(response, Bytes::from_static(b"ping"));
    Ok(())
}
```

其中：

- `Server::bind` 用于创建并绑定本地 IPC 服务端
- `server.accept` 用于接受来自其他进程的连接
- `connect` 用于连接到 puniyu IPC 服务端
- `send_bytes` 与 `recv_bytes` 使用 4 字节大端长度前缀收发 `Bytes` 消息
