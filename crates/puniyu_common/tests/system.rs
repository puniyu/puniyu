use puniyu_common::system::SystemInfo;

#[test]
fn test_bot_info() {
	let bot_info = SystemInfo::new();

	assert!(bot_info.pid > 0);

	std::thread::sleep(std::time::Duration::from_secs(1));

	let bot_info = SystemInfo::new();
	assert!(bot_info.run_time > 0);

	assert!(bot_info.used_memory >= 0.0);

	if let Some(cpu_usage) = bot_info.cpu_usage {
		assert!(cpu_usage <= 100f32);
	}

	if let Some(memory_usage) = bot_info.memory_usage {
		assert!(memory_usage <= 100f32);
	}
}
