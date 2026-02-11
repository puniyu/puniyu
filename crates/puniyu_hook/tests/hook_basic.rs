use async_trait::async_trait;
use puniyu_context::EventContext;
use puniyu_hook::Hook;
use puniyu_hook::types::{HookEventType, HookType};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

// 简单的测试钩子
struct TestHook {
	name: &'static str,
	hook_type: HookType,
	rank: u32,
	executed: Arc<AtomicBool>,
}

#[async_trait]
impl Hook for TestHook {
	fn name(&self) -> &'static str {
		self.name
	}

	fn r#type(&self) -> &HookType {
		&self.hook_type
	}

	fn rank(&self) -> u32 {
		self.rank
	}

	async fn run(&self, _ctx: Option<&EventContext>) -> puniyu_error::Result {
		self.executed.store(true, Ordering::SeqCst);
		Ok(())
	}
}

#[test]
fn test_hook_name() {
	let executed = Arc::new(AtomicBool::new(false));
	let hook = TestHook {
		name: "test_hook",
		hook_type: HookType::Event(HookEventType::Message),
		rank: 100,
		executed,
	};

	assert_eq!(hook.name(), "test_hook");
}

#[test]
fn test_hook_type() {
	let executed = Arc::new(AtomicBool::new(false));
	let hook = TestHook {
		name: "test_hook",
		hook_type: HookType::Event(HookEventType::Message),
		rank: 100,
		executed,
	};

	assert_eq!(hook.r#type(), &HookType::Event(HookEventType::Message));
}

#[test]
fn test_hook_rank() {
	let executed = Arc::new(AtomicBool::new(false));
	let hook = TestHook {
		name: "test_hook",
		hook_type: HookType::Event(HookEventType::Message),
		rank: 50,
		executed,
	};

	assert_eq!(hook.rank(), 50);
}

#[tokio::test]
async fn test_hook_run() {
	let executed = Arc::new(AtomicBool::new(false));
	let hook = TestHook {
		name: "test_hook",
		hook_type: HookType::Event(HookEventType::Message),
		rank: 100,
		executed: executed.clone(),
	};

	assert!(!executed.load(Ordering::SeqCst));

	let result = hook.run(None).await;
	assert!(result.is_ok());
	assert!(executed.load(Ordering::SeqCst));
}

#[tokio::test]
async fn test_hook_multiple_runs() {
	let counter = Arc::new(AtomicU32::new(0));

	struct CounterHook {
		counter: Arc<AtomicU32>,
	}

	#[async_trait]
	impl Hook for CounterHook {
		fn name(&self) -> &'static str {
			"counter_hook"
		}

		fn r#type(&self) -> &HookType {
			&HookType::Event(HookEventType::Message)
		}

		fn rank(&self) -> u32 {
			100
		}

		async fn run(&self, _ctx: Option<&EventContext>) -> puniyu_error::Result {
			self.counter.fetch_add(1, Ordering::SeqCst);
			Ok(())
		}
	}

	let hook = CounterHook { counter: counter.clone() };

	assert_eq!(counter.load(Ordering::SeqCst), 0);

	hook.run(None).await.unwrap();
	assert_eq!(counter.load(Ordering::SeqCst), 1);

	hook.run(None).await.unwrap();
	assert_eq!(counter.load(Ordering::SeqCst), 2);

	hook.run(None).await.unwrap();
	assert_eq!(counter.load(Ordering::SeqCst), 3);
}

#[tokio::test]
async fn test_hook_error_handling() {
	struct ErrorHook;

	#[async_trait]
	impl Hook for ErrorHook {
		fn name(&self) -> &'static str {
			"error_hook"
		}

		fn r#type(&self) -> &HookType {
			&HookType::Event(HookEventType::Message)
		}

		fn rank(&self) -> u32 {
			100
		}

		async fn run(&self, _ctx: Option<&EventContext>) -> puniyu_error::Result {
			Err("Test error".into())
		}
	}

	let hook = ErrorHook;
	let result = hook.run(None).await;

	assert!(result.is_err());
}

#[test]
fn test_hook_priority_comparison() {
	let executed1 = Arc::new(AtomicBool::new(false));
	let hook1 = TestHook {
		name: "high_priority",
		hook_type: HookType::Event(HookEventType::Message),
		rank: 10,
		executed: executed1,
	};

	let executed2 = Arc::new(AtomicBool::new(false));
	let hook2 = TestHook {
		name: "low_priority",
		hook_type: HookType::Event(HookEventType::Message),
		rank: 100,
		executed: executed2,
	};

	assert!(hook1.rank() < hook2.rank());
}

#[test]
fn test_hook_different_types() {
	let executed1 = Arc::new(AtomicBool::new(false));
	let hook1 = TestHook {
		name: "message_hook",
		hook_type: HookType::Event(HookEventType::Message),
		rank: 100,
		executed: executed1,
	};

	let executed2 = Arc::new(AtomicBool::new(false));
	let hook2 = TestHook {
		name: "notion_hook",
		hook_type: HookType::Event(HookEventType::Notion),
		rank: 100,
		executed: executed2,
	};

	assert_ne!(hook1.r#type(), hook2.r#type());
}

#[tokio::test]
async fn test_hook_with_state() {
	use std::sync::Mutex;

	struct StatefulHook {
		state: Arc<Mutex<Vec<String>>>,
	}

	#[async_trait]
	impl Hook for StatefulHook {
		fn name(&self) -> &'static str {
			"stateful_hook"
		}

		fn r#type(&self) -> &HookType {
			&HookType::Event(HookEventType::Message)
		}

		fn rank(&self) -> u32 {
			100
		}

		async fn run(&self, _ctx: Option<&EventContext>) -> puniyu_error::Result {
			let mut state = self.state.lock().unwrap();
			state.push("executed".to_string());
			Ok(())
		}
	}

	let state = Arc::new(Mutex::new(Vec::new()));
	let hook = StatefulHook { state: state.clone() };

	assert_eq!(state.lock().unwrap().len(), 0);

	hook.run(None).await.unwrap();
	assert_eq!(state.lock().unwrap().len(), 1);

	hook.run(None).await.unwrap();
	assert_eq!(state.lock().unwrap().len(), 2);
}

#[tokio::test]
async fn test_hook_async_operation() {
	use tokio::time::{Duration, sleep};

	struct AsyncHook {
		completed: Arc<AtomicBool>,
	}

	#[async_trait]
	impl Hook for AsyncHook {
		fn name(&self) -> &'static str {
			"async_hook"
		}

		fn r#type(&self) -> &HookType {
			&HookType::Event(HookEventType::Message)
		}

		fn rank(&self) -> u32 {
			100
		}

		async fn run(&self, _ctx: Option<&EventContext>) -> puniyu_error::Result {
			sleep(Duration::from_millis(10)).await;
			self.completed.store(true, Ordering::SeqCst);
			Ok(())
		}
	}

	let completed = Arc::new(AtomicBool::new(false));
	let hook = AsyncHook { completed: completed.clone() };

	assert!(!completed.load(Ordering::SeqCst));

	hook.run(None).await.unwrap();

	assert!(completed.load(Ordering::SeqCst));
}

#[test]
fn test_hook_send_sync() {
	fn assert_send<T: Send>() {}
	fn assert_sync<T: Sync>() {}

	assert_send::<Box<dyn Hook>>();
	assert_sync::<Box<dyn Hook>>();
}
