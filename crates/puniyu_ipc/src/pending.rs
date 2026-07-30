use std::sync::atomic::{AtomicU32, Ordering};

use scc::HashMap;
use tokio::sync::oneshot;

use crate::frame::Response;

/// 请求等待表。
///
/// 发送 Request 时分配 id + 创建 oneshot channel，
/// 收到 Response 时按 id 唤醒。
pub struct Pending {
	map: HashMap<u32, oneshot::Sender<Response>>,
	next_id: AtomicU32,
}

impl Pending {
	pub fn new() -> Self {
		Self {
			map: HashMap::new(),
			next_id: AtomicU32::new(0),
		}
	}

	/// 分配一个新的请求 ID
	pub fn insert(&self) -> (u32, oneshot::Receiver<Response>) {
		let id = self.next_id.fetch_add(1, Ordering::Relaxed);
		let (tx, rx) = oneshot::channel();
		self.map.insert_sync(id, tx).ok();
		(id, rx)
	}

	/// 收到 Response 时，唤醒对应的等待者
	pub fn complete(&self, response: Response) -> bool {
		if let Some((_, tx)) = self.map.remove_sync(&response.id) {
			let _ = tx.send(response);
			true
		} else {
			false
		}
	}
}
