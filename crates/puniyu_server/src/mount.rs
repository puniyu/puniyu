use crate::Error;
use crate::server::{MountContent, Server};
use std::sync::Weak;

pub struct HttpMount {
	server: Weak<Server>,
	content: MountContent,
	id: Option<u64>,
}

impl HttpMount {
	pub(crate) fn new(server: Weak<Server>, content: MountContent) -> Self {
		Self { server, content, id: None }
	}

	pub fn mount(&mut self) -> Result<(), Error> {
		if self.id.is_some() {
			return Err(Error::AlreadyMounted);
		}
		let server = self.server.upgrade().ok_or(Error::ServerUnavailable)?;
		self.id = Some(server.mount(self.content.clone())?);
		Ok(())
	}

	pub fn unmount(&mut self) {
		let Some(id) = self.id else {
			return;
		};
		if let Some(server) = self.server.upgrade() {
			server.unmount(id);
		}
		self.id = None;
	}

	pub fn is_mounted(&self) -> bool {
		self.id.is_some() && self.server.strong_count() > 0
	}
}

impl Drop for HttpMount {
	fn drop(&mut self) {
		self.unmount();
	}
}
