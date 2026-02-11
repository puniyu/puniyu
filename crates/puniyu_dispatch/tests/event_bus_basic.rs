use puniyu_dispatch::EventBus;

#[test]
fn test_event_bus_creation() {
	let event_bus = EventBus::new();
	assert!(!event_bus.is_running());
}

#[test]
fn test_event_bus_with_capacity() {
	let event_bus = EventBus::with_capacity(1000);
	assert!(!event_bus.is_running());
}

#[test]
fn test_event_bus_default() {
	let event_bus = EventBus::default();
	assert!(!event_bus.is_running());
}

#[test]
fn test_event_bus_sender() {
	let event_bus = EventBus::new();
	let _sender = event_bus.sender();
}

#[test]
fn test_event_bus_is_running_before_run() {
	let event_bus = EventBus::new();
	assert!(!event_bus.is_running());
}

#[tokio::test]
async fn test_event_bus_run() {
	let event_bus = EventBus::new();
	let handle = event_bus.run();

	assert!(event_bus.is_running());

	event_bus.shutdown();
	let _ = tokio::time::timeout(tokio::time::Duration::from_secs(1), handle).await;
}

#[tokio::test]
async fn test_event_bus_shutdown() {
	let event_bus = EventBus::new();
	let handle = event_bus.run();

	event_bus.shutdown();

	let result = tokio::time::timeout(tokio::time::Duration::from_secs(1), handle).await;
	assert!(result.is_ok());
}

#[tokio::test]
async fn test_event_bus_multiple_senders() {
	let event_bus = EventBus::new();
	let _sender1 = event_bus.sender();
	let _sender2 = event_bus.sender();
	let _sender3 = event_bus.sender();
}

#[test]
fn test_event_bus_debug() {
	let event_bus = EventBus::new();
	let debug_str = format!("{:?}", event_bus);
	assert!(debug_str.contains("EventBus"));
}

#[tokio::test]
async fn test_event_bus_capacity_zero() {
	let event_bus = EventBus::with_capacity(0);
	assert!(!event_bus.is_running());
}

#[tokio::test]
async fn test_event_bus_large_capacity() {
	let event_bus = EventBus::with_capacity(100000);
	assert!(!event_bus.is_running());
}
