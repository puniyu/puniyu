use puniyu_common::system::SystemInfo;

#[test]
fn test_host_info() {
	let host_info = SystemInfo::host();

	assert!(!host_info.host_name.is_empty());
	assert!(!host_info.os_name.is_empty());
	assert!(!host_info.os_version.is_empty());
	assert!(!host_info.os_type.is_empty());
	assert!(host_info.boot_time > 0);
}

#[test]
fn test_cpu_info() {
	let cpu_info = SystemInfo::cpu();

	assert!(!cpu_info.cpu_model.is_empty());
	assert!(cpu_info.cpu_cores > 0);

	if let Some(frequency) = cpu_info.cpu_frequency {
		assert!(frequency > 0.0);
	}

	if let Some(usage) = cpu_info.cpu_usage {
		assert!(usage <= 100);
	}
}

#[test]
fn test_bot_info() {
	let bot_info = SystemInfo::bot();

	assert!(bot_info.pid.as_u32() > 0);

	std::thread::sleep(std::time::Duration::from_secs(1));

	let bot_info = SystemInfo::bot();
	assert!(bot_info.run_time > 0);

	assert!(bot_info.used_memory >= 0.0);

	if let Some(cpu_usage) = bot_info.cpu_usage {
		assert!(cpu_usage <= 100);
	}

	if let Some(memory_usage) = bot_info.memory_usage {
		assert!(memory_usage <= 100);
	}
}

#[test]
fn test_memory_info() {
	let memory_info = SystemInfo::memory();

	assert!(memory_info.total_memory > 0.0);
	assert!(memory_info.used_memory >= 0.0);
	assert!(memory_info.free_memory >= 0.0);

	if let Some(usage) = memory_info.memory_usage {
		assert!(usage >= 0.0 as u8 && usage <= 100.0 as u8);
	}
}

#[test]
fn test_disk_info() {
	let disk_info = SystemInfo::disk();

	assert!(disk_info.total_disk_space > 0.0);
	assert!(disk_info.total_used_space >= 0.0);
	assert!(disk_info.total_free_space >= 0.0);

	for disk in &disk_info.disks {
		assert!(!disk.name.is_empty());
		assert!(disk.usage >= 0.0 && disk.usage <= 100.0);
		assert!(disk.total_space > 0.0);
		assert!(disk.used_space >= 0.0);
	}
}

#[test]
fn test_gpu_info() {
	let gpu_info = SystemInfo::gpu();

	if let Some(gpu) = gpu_info {
		assert!(!gpu.gpu_model.is_empty());
		assert!(gpu.gpu_memory_total > 0.0);
		assert!(gpu.gpu_memory_used >= 0.0);
		assert!(gpu.gpu_memory_free >= 0.0);
		assert!(gpu.gpu_usage <= 100);
	} else {
		dbg!("未检测到GPU");
	}
}
