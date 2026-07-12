use std::sync::{Arc, LazyLock, Mutex};

use salvo::{Handler, Router};

#[derive(Default)]
pub(crate) struct Inner {
	pub routers: Vec<Router>,
	pub hoops: Vec<Arc<dyn Handler>>,
}

impl Inner {
    #[inline]
	pub fn new() -> Self {
		Self::default()
	}
}

pub(crate) static INNER: LazyLock<Mutex<Inner>> = LazyLock::new(|| Mutex::new(Inner::new()));

pub struct App;

impl App {
	pub fn hoop<H>(hoop: H)
	where
		H: Handler + 'static,
	{
		let mut guard = INNER.lock().expect("router mutex poisoned");
		guard.hoops.push(Arc::new(hoop));
	}

	pub fn router(router: Router) {
		let mut guard = INNER.lock().expect("router mutex poisoned");
		guard.routers.push(router);
	}
}


pub(crate) fn take() -> Inner {
    let mut guard = INNER.lock().expect("router mutex poisoned");
    std::mem::take(&mut *guard)
}