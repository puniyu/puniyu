use scc::HashMap;
use std::sync::Arc;

use bytes::Bytes;
use serde::{de::DeserializeOwned, Serialize};
use smol_str::SmolStr;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::sync::Mutex;

use crate::codec::{read_frame, write_frame};
use crate::error::Error;
use crate::frame::{Event, Frame, Notify, Request, Response};
use crate::pending::Pending;
use crate::handler::{EventHandler, ServiceHandler};
use crate::ServiceName;

/// IPC 双向端点。
///
/// 既是传输层，也是服务注册中心。
/// 插件直接在 Endpoint 上注册和调用服务。
pub struct Endpoint<R, W> {
	reader: Arc<Mutex<R>>,
	writer: Arc<Mutex<W>>,
	services: HashMap<ServiceName, Arc<dyn ServiceHandler>>,
	pending: Pending,
	event_handler: Option<Arc<dyn EventHandler>>,
}

impl<R: AsyncRead + Unpin, W: AsyncWrite + Unpin> Endpoint<R, W> {
	/// 创建新的端点
	pub fn new(reader: R, writer: W) -> Self {
		Self {
			reader: Arc::new(Mutex::new(reader)),
			writer: Arc::new(Mutex::new(writer)),
			services: HashMap::new(),
			pending: Pending::new(),
			event_handler: None,
		}
	}

	/// 注册 IPC 服务（支持运行时热注册）
	pub fn register(&self, name: impl Into<ServiceName>, handler: impl ServiceHandler + 'static) {
		self.services.insert_sync(name.into(), Arc::new(handler)).ok();
	}

	/// 注册事件处理器
	pub fn on_event(&mut self, handler: impl EventHandler + 'static) {
		self.event_handler = Some(Arc::new(handler));
	}

	/// 调用远程服务（等待响应）
	pub async fn call<P: Serialize, T: DeserializeOwned>(
		&self,
		service: &str,
		params: P,
	) -> Result<T, Error> {
		let payload = Bytes::from(rmp_serde::to_vec(&params).map_err(Error::Encode)?);
		let (id, rx) = self.pending.insert();

		let req = Request::new(
			id,
			ServiceName::new(service),
			payload,
		);

		{
			let mut writer = self.writer.lock().await;
			write_frame(&mut *writer, &Frame::Request(req)).await?;
		}

		let response = rx.await.map_err(|_| Error::ChannelClosed)?;

		if response.success {
			rmp_serde::from_slice(&response.payload).map_err(Error::Decode)
		} else {
			Err(Error::Remote(response.error.unwrap_or_else(|| "unknown error".into()).to_string()))
		}
	}

	/// 发送通知
	pub async fn notify<P: Serialize>(
		&self,
		service: &str,
		params: P,
	) -> Result<(), Error> {
		let payload = Bytes::from(rmp_serde::to_vec(&params).map_err(Error::Encode)?);
		let notif = Notify::new(
			ServiceName::new(service),
			payload,
		);

		let mut writer = self.writer.lock().await;
		write_frame(&mut *writer, &Frame::Notify(notif)).await
	}

	/// 广播事件
	pub async fn emit<P: Serialize>(
		&self,
		event: &str,
		data: P,
	) -> Result<(), Error> {
		let payload = Bytes::from(rmp_serde::to_vec(&data).map_err(Error::Encode)?);
		let ev = Event::new(SmolStr::new(event), payload);

		let mut writer = self.writer.lock().await;
		write_frame(&mut *writer, &Frame::Event(ev)).await
	}

	/// 启动接收循环（处理收到的 Request/Response/Notify/Event）
	pub async fn serve(&self) -> Result<(), Error> {
		loop {
			let frame = {
				let mut reader = self.reader.lock().await;
				read_frame(&mut *reader).await?
			};

			match frame {
				Frame::Request(req) => {
					self.handle_request(req).await;
				}
				Frame::Response(res) => {
					self.pending.complete(res);
				}
				Frame::Notify(notif) => {
					self.handle_notify(notif).await;
				}
				Frame::Event(ev) => {
					self.handle_event(ev).await;
				}
			}
		}
	}

	/// 处理收到的请求
	async fn handle_request(&self, req: Request) {
		let handler = match self.services.read_sync(&req.service, |_, v| v.clone()) {
			Some(h) => h,
			None => {
				let res = Response::error(req.id, format!("unknown service: {}", req.service.as_str()));
				let _ = self.send_response(res).await;
				return;
			}
		};

		let result = handler.handle(req.service.as_str(), req.payload).await;

		let res = match result {
			Ok(payload) => Response::success(req.id, payload),
			Err(err) => Response::error(req.id, err.to_string()),
		};

		let _ = self.send_response(res).await;
	}

	/// 处理收到的通知
	async fn handle_notify(&self, notif: Notify) {
		if let Some(handler) = self.services.read_sync(&notif.service, |_, v| v.clone()) {
			let _ = handler.handle(notif.service.as_str(), notif.payload).await;
		}
	}

	/// 处理收到的事件
	async fn handle_event(&self, ev: Event) {
		if let Some(handler) = &self.event_handler {
			handler.handle(ev.event.as_str(), ev.payload).await;
		}
	}

	/// 发送响应
	async fn send_response(&self, res: Response) -> Result<(), Error> {
		let mut writer = self.writer.lock().await;
		write_frame(&mut *writer, &Frame::Response(res)).await
	}
}
