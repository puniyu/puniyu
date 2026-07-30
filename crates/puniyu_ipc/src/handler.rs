use async_trait::async_trait;
use bytes::Bytes;
use puniyu_error::AnyError;

/// IPC 服务处理器。
///
/// 插件实现此 trait 来处理来自对端的 IPC 请求和通知。
/// Endpoint 收到 Request/Notify 时按 service name 路由到对应 handler。
#[async_trait]
pub trait ServiceHandler: Send + Sync {
	/// 处理 IPC 请求/通知
	///
	/// - `service`: 目标服务名
	/// - `payload`: msgpack 编码的参数
	async fn handle(&self, service: &str, payload: Bytes) -> AnyError<Bytes>;
}

/// 事件处理器。
///
/// Endpoint 收到 Event 帧时调用。用于一对多广播场景。
#[async_trait]
pub trait EventHandler: Send + Sync {
	/// 处理事件
	///
	/// - `event`: 事件名称
	/// - `payload`: msgpack 编码的事件数据
	async fn handle(&self, event: &str, payload: Bytes);
}
