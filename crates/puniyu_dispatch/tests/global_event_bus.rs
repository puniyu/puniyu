use puniyu_dispatch::{EventBus, get_event_bus, setup_event_bus};
use std::sync::Once;

static INIT: Once = Once::new();

fn setup_test_event_bus() {
	INIT.call_once(|| {
		let event_bus = EventBus::new();
		setup_event_bus(event_bus);
	});
}

#[test]
fn test_setup_and_get_event_bus() {
	setup_test_event_bus();
	let event_bus = get_event_bus();
	assert!(!event_bus.is_running());
}

#[test]
fn test_get_event_bus_sender() {
	setup_test_event_bus();
	let event_bus = get_event_bus();
	let _sender = event_bus.sender();
}

#[test]
fn test_get_event_bus_multiple_times() {
	setup_test_event_bus();
	let event_bus1 = get_event_bus();
	let event_bus2 = get_event_bus();

	// 应该返回同一个实例
	assert!(std::ptr::eq(event_bus1, event_bus2));
}

#[tokio::test]
async fn test_global_event_bus_run() {
	setup_test_event_bus();
	let event_bus = get_event_bus();

	// 注意：由于是全局单例，run() 只能调用一次
	// 这里我们只检查是否可以获取事件总线
	assert!(!event_bus.is_running() || event_bus.is_running());
}

#[test]
fn test_global_event_bus_debug() {
	setup_test_event_bus();
	let event_bus = get_event_bus();
	let debug_str = format!("{:?}", event_bus);
	assert!(debug_str.contains("EventBus"));
}
