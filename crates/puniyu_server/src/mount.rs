use crate::Error;
use crate::http::{HttpInner, MountId};
use std::sync::Weak;

pub struct HttpMount {
	pub(crate) inner: Weak<HttpInner>,
	pub(crate) id: MountId,
	pub(crate) mounted: bool,
}

impl HttpMount {
	pub fn unmount(mut self) -> Result<(), Error> {
		if let Some(inner) = self.inner.upgrade() {
			inner.unmount(self.id)?;
		}
		self.mounted = false;
		Ok(())
	}
}

impl Drop for HttpMount {
	fn drop(&mut self) {
		if !self.mounted {
			return;
		}
		if let Some(inner) = self.inner.upgrade() {
			let _ = inner.unmount(self.id);
		}
	}
}
