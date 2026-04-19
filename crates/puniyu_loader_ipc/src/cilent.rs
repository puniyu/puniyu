use std::time::Duration;

use puniyu_ipc::Message;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::time::timeout;
use uuid::Uuid;

use crate::service::Service;

const TIMEOUT: Duration = Duration::from_secs(3);

pub struct Client;

impl Client {
	pub async fn send<P: Serialize, D: DeserializeOwned>(
		&self,
		service: Service,
		params: P,
	) -> puniyu_ipc::Result<D> {
		use std::io::{Error, ErrorKind};
		let id = Uuid::new_v4().as_u64_pair().0;
		let request = Message::request(id, service.name(), params)?;
		let mut connection = timeout(TIMEOUT, puniyu_ipc::connect())
			.await
			.map_err(|_| Error::new(ErrorKind::TimedOut, "connect timeout"))??;
		connection.send(&request).await?;
		let message = timeout(TIMEOUT, connection.recv())
			.await
			.map_err(|_| Error::new(ErrorKind::TimedOut, "receive timeout"))??;
		match message {
			Message::Response(response) => response.decode_result::<D>(),
			Message::Error(error) => Err(Error::other(format!(
				"{}: {}",
				error.error.code, error.error.message
			))
			.into()),
			other => Err(Error::new(
				ErrorKind::InvalidData,
				format!("unexpected message kind: {:?}", other.kind()),
			)
			.into()),
		}
	}
}
