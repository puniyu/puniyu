use punicore_utils::system::system_info::{
    get_bot_info, get_cpu_info, get_disk_info, get_memory_info, get_system_info,
};

#[test]
fn get_system_info_test() {
    let system_info = get_system_info();

    // 验证系统信息结构不为空
    assert!(!system_info.cpu.cpu_model.is_empty());
    assert!(system_info.cpu.cpu_cores > 0);
    assert!(system_info.cpu.cpu_frequency >= 0.0);
    assert!(system_info.cpu.cpu_usage >= 0.0);

    // 验证内存信息
    assert!(system_info.memory.total_memory > 0.0);
    assert!(system_info.memory.used_memory >= 0.0);
    assert!(system_info.memory.free_memory >= 0.0);
    assert!(system_info.memory.memory_usage >= 0.0 && system_info.memory.memory_usage <= 100.0);

    // 验证磁盘信息
    assert!(system_info.disk.total_disk_space >= 0.0);
    assert!(system_info.disk.total_used_space >= 0.0);
    assert!(system_info.disk.total_free_space >= 0.0);
    assert!(system_info.disk.total_usage >= 0.0 && system_info.disk.total_usage <= 100.0);

    // 验证Bot信息
    assert!(system_info.bot.cpu_usage >= 0.0);
    assert!(system_info.bot.memory_usage >= 0.0);
}

#[test]
fn get_cpu_info_test() {
    let cpu_info = get_cpu_info();

    assert!(!cpu_info.cpu_model.is_empty());
    assert!(cpu_info.cpu_cores > 0);
    assert!(cpu_info.cpu_frequency >= 0.0);
    assert!(cpu_info.cpu_usage >= 0.0 && cpu_info.cpu_usage <= 100.0);
}

#[test]
fn get_memory_info_test() {
    let memory_info = get_memory_info();

    assert!(memory_info.total_memory > 0.0);
    assert!(memory_info.used_memory >= 0.0);
    assert!(memory_info.free_memory >= 0.0);
    assert!(memory_info.memory_usage >= 0.0 && memory_info.memory_usage <= 100.0);
}

#[test]
fn get_disk_info_test() {
    let disk_info = get_disk_info();

    assert!(disk_info.total_disk_space >= 0.0);
    assert!(disk_info.total_used_space >= 0.0);
    assert!(disk_info.total_free_space >= 0.0);
    assert!(disk_info.total_usage >= 0.0 && disk_info.total_usage <= 100.0);

    assert!(!disk_info.disks.is_empty());

    // 验证每个磁盘的信息
    for disk in &disk_info.disks {
        assert!(!disk.name.is_empty());
        assert!(disk.total_space >= 0.0);
        assert!(disk.used_space >= 0.0);
        assert!(disk.free_space >= 0.0);
        assert!(disk.usage >= 0.0 && disk.usage <= 100.0);

        assert!(
            disk.total_space >= disk.used_space,
            "Disk {}: total space ({:.2}) should be >= used space ({:.2})",
            disk.name,
            disk.total_space,
            disk.used_space
        );
        assert!(
            disk.total_space >= disk.free_space,
            "Disk {}: total space ({:.2}) should be >= free space ({:.2})",
            disk.name,
            disk.total_space,
            disk.free_space
        );
    }
}

#[test]
fn get_bot_info_test() {
    let bot_info = get_bot_info();

    assert!(bot_info.cpu_usage >= 0.0);
    assert!(bot_info.memory_usage >= 0.0);
    assert!(bot_info.used_memory >= 0.0);
    assert!(bot_info.memory_usage >= 0.0 && bot_info.memory_usage <= 100.0);
}
