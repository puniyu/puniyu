# puniyu_ipc

本地 IPC 协议库，提供基于 `interprocess` local socket 的 Tokio 连接能力，以及面向 MessagePack 协议消息的双向收发接口。

## 特征

- 提供基于 namespaced local socket 的本地 IPC 监听与连接能力
- 内建 `u32 frame_len | u8 version | u8 kind | payload(msgpack)` 帧格式
- 对外暴露 typed message API，而不是裸 bytes 收发
- 支持 `request / response / error / cancel / event` 五类协议消息
- 内置 8 MiB 单帧大小限制，避免异常大消息导致内存分配失控

## 快速开始

可以先从服务端监听和客户端连接的最小示例开始：

```rust
async fn server() -> Result<(), puniyu_ipc::Error> {
    let server = puniyu_ipc::Server::bind().await?;
    let mut connection = server.accept().await?;

    match connection.recv().await? {
        puniyu_ipc::Message::Request(request) => {
            let response = puniyu_ipc::Message::response(request.id, "pong")?;
            connection.send(&response).await?;
        }
        _ => {}
    }

    Ok(())
}

async fn client() -> Result<(), puniyu_ipc::Error> {
    let mut connection = puniyu_ipc::connect().await?;

    let request = puniyu_ipc::Message::request(1, "plugin.command.ping", ["hello"])?;
    connection.send(&request).await?;

    match connection.recv().await? {
        puniyu_ipc::Message::Response(response) => {
            let result: String = response.decode_result()?;
            assert_eq!(result, "pong");
        }
        other => panic!("unexpected response: {:?}", other.kind()),
    }

    Ok(())
}
```

其中：

- `Server::bind` 用于创建并绑定本地 IPC 服务端
- `server.accept` 用于接受来自其他进程的连接并返回 `Connection`
- `connect` 用于连接到 puniyu IPC 服务端
- `Connection::send` 与 `Connection::recv` 直接收发 `Message`
- `Message::request/response/error/cancel/event` 用于构造协议消息
