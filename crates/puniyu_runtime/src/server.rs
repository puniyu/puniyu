use std::io;

use actix_web::dev::ServerHandle;
use tokio::task::JoinHandle;

pub struct ServerRuntime {
	handle: ServerHandle,
	join_handle: JoinHandle<io::Result<()>>,
}

impl ServerRuntime {
	pub fn new(handle: ServerHandle, join_handle: JoinHandle<io::Result<()>>) -> Self {
		Self { handle, join_handle }
	}

	pub async fn stop(&self) -> io::Result<()> {
		self.handle.stop(true).await;
		Ok(())
	}

	pub async fn wait(self) -> io::Result<()> {
		self.join_handle
			.await
			.map_err(|e| io::Error::other(format!("Server task join error: {}", e)))?
	}

	pub async fn shutdown(self) -> io::Result<()> {
		self.stop().await?;
		self.wait().await
	}
}
