use crate::Error;
use crate::http::{HttpInner, MountContent, MountId};
use std::sync::{Arc, Weak};

pub struct HttpMount {
	inner: Weak<HttpInner>,
	content: MountContent,
	id: Option<MountId>,
}

impl HttpMount {
	pub(crate) fn new(inner: &Arc<HttpInner>, content: MountContent) -> Self {
		Self { inner: Arc::downgrade(inner), content, id: None }
	}

	pub fn mount(&mut self) -> Result<(), Error> {
		if self.id.is_some() {
			return Err(Error::AlreadyMounted);
		}
		let inner = self.inner.upgrade().ok_or(Error::HttpUnavailable)?;
		self.id = Some(inner.mount(self.content.clone())?);
		Ok(())
	}

	pub fn unmount(&mut self) {
		let Some(id) = self.id else {
			return;
		};
		let Some(inner) = self.inner.upgrade() else {
			self.id = None;
			return;
		};
		inner.unmount(id);
		self.id = None;
	}

	pub fn is_mounted(&self) -> bool {
		self.id.is_some() && self.inner.strong_count() > 0
	}
}

impl Drop for HttpMount {
	fn drop(&mut self) {
		self.unmount();
	}
}
