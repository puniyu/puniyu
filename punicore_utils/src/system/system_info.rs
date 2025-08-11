use crate::utils::number::format_to_f32;
use rust_decimal::prelude::*;
use std::process;
use std::thread::sleep;
use std::time::Duration;
use sysinfo::{Disks, Pid, ProcessesToUpdate, System};

#[derive(Debug)]
pub struct SystemInfo {
    /// CPU信息
    pub cpu: CpuInfo,
    /// 内存信息
    pub memory: MemoryInfo,
    /// 硬盘信息
    pub disk: DiskInfo,
    /// 系统信息
    pub bot: BotInfo,
}
#[derive(Debug)]
pub struct CpuInfo {
    /// CPU型号
    pub cpu_model: String,
    /// CPU核心数
    pub cpu_cores: usize,
    /// CPU频率(单位: GHz)
    pub cpu_frequency: f32,
    /// CPU使用率
    pub cpu_usage: f32,
}

#[derive(Debug)]
pub struct MemoryInfo {
    /// 总内存(单位: MB)
    pub total_memory: f32,
    /// 已用内存(单位: MB)
    pub used_memory: f32,
    /// 可用内存(单位: MB)
    pub free_memory: f32,
    /// 内存使用率
    pub memory_usage: f32,
}

#[derive(Debug)]
pub struct DiskDetail {
    /// 磁盘名称
    pub name: String,
    /// 总磁盘空间(单位: GB)
    pub total_space: f32,
    /// 已用磁盘空间(单位: GB)
    pub used_space: f32,
    /// 可用磁盘空间(单位: GB)
    pub free_space: f32,
    /// 磁盘使用率
    pub usage: f32,
}

#[derive(Debug)]
pub struct DiskInfo {
    /// 总磁盘空间(单位: GB)
    pub total_disk_space: f32,
    /// 总已用磁盘空间(单位: GB)
    pub total_used_space: f32,
    /// 总可用磁盘空间(单位: GB)
    pub total_free_space: f32,
    /// 总体磁盘使用率
    pub total_usage: f32,
    /// 各个磁盘详细信息
    pub disks: Vec<DiskDetail>,
}

#[derive(Debug)]
pub struct BotInfo {
    /// CPU使用率
    pub cpu_usage: f32,
    /// 内存使用率
    pub memory_usage: f32,
    /// 已用内存(单位: MB)
    pub used_memory: f32,
}

/// 获取系统信息
///
/// 此函数可以获取系统信息，包括CPU、内存、磁盘、Bot信息等
/// # 返回值
///
/// * [SystemInfo] - 系统信息
///
pub fn get_system_info() -> SystemInfo {
    let cpu = get_cpu_info();
    let memory = get_memory_info();
    let disk = get_disk_info();
    let bot = get_bot_info();
    SystemInfo {
        cpu,
        memory,
        disk,
        bot,
    }
}

/// 获取CPU信息
///
/// 此函数可以获取CPU信息，包括型号、核心数、频率、使用率等
/// # 返回值
///
/// * [CpuInfo] - CPU信息
///
pub fn get_cpu_info() -> CpuInfo {
    let mut system = System::new();
    system.refresh_cpu_all();

    sleep(Duration::from_millis(1000));
    system.refresh_cpu_usage();

    let cpus = system.cpus();

    let cpu_usage = if !cpus.is_empty() {
        let usage = cpus.iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / cpus.len() as f32;
        let decimal_value = Decimal::from_f32(usage).unwrap_or(Decimal::ZERO);
        let rounded = decimal_value.round_dp(2);
        rounded.to_f32().unwrap_or(0.0)
    } else {
        0.0
    };

    let cpu_cores = cpus.len();

    let cpu_model = if !cpus.is_empty() {
        cpus[0].brand().to_string()
    } else {
        String::new()
    };

    let cpu_frequency = if !cpus.is_empty() {
        cpus[0].frequency() as f32
    } else {
        0.0
    };

    CpuInfo {
        cpu_usage,
        cpu_frequency,
        cpu_cores,
        cpu_model,
    }
}

/// 获取内存信息
///
/// 此函数可以获取内存信息，包括总内存、已用内存、可用内存、内存使用率等
/// # 返回值
///
/// * [MemoryInfo] - 内存信息
///
pub fn get_memory_info() -> MemoryInfo {
    let mut system = System::new();
    system.refresh_memory();

    let total_memory = system.total_memory() / 1024 / 1024;
    let used_memory = system.used_memory() / 1024 / 1024;
    let free_memory = system.free_memory() / 1024 / 1024;

    let total_memory_f32 = format_to_f32(total_memory as f32, 2);
    let used_memory_f32 = format_to_f32(used_memory as f32, 2);
    let free_memory_f32 = format_to_f32(free_memory as f32, 2);

    let memory_usage = {
        let memory_value = Decimal::from_f32((used_memory as f32 / total_memory as f32) * 100.0)
            .unwrap_or(Decimal::ZERO);
        let rounded = memory_value.round_dp(2);
        rounded.to_f32().unwrap_or(0.0)
    };

    MemoryInfo {
        total_memory: total_memory_f32,
        used_memory: used_memory_f32,
        free_memory: free_memory_f32,
        memory_usage,
    }
}

/// 获取磁盘信息
///
/// 此函数可以获取磁盘信息，包括总磁盘空间、已用磁盘空间、可用磁盘空间、磁盘使用率等
/// # 返回值
///
/// * [DiskInfo] - 磁盘信息
///
pub fn get_disk_info() -> DiskInfo {
    let disks = Disks::new_with_refreshed_list();

    let mut total_disk_space = 0f32;
    let mut total_used_space = 0f32;
    let mut total_free_space = 0f32;
    let mut disk_details = Vec::new();

    for disk in disks.list() {
        let total_space = disk.total_space() as f32 / (1024.0 * 1024.0 * 1024.0);
        let free_space = disk.available_space() as f32 / (1024.0 * 1024.0 * 1024.0);
        let used_space = total_space - free_space;

        let usage = if disk.total_space() > 0 {
            (used_space / total_space) * 100.0
        } else {
            0.0
        };

        let disk_detail = DiskDetail {
            name: disk.name().to_string_lossy().to_string(),
            total_space: format_to_f32(total_space, 2),
            used_space: format_to_f32(used_space, 2),
            free_space: format_to_f32(free_space, 2),
            usage: format_to_f32(usage, 2),
        };

        total_disk_space += total_space;
        total_used_space += used_space;
        total_free_space += free_space;
        disk_details.push(disk_detail);
    }

    let total_usage = if total_disk_space > 0.0 {
        (total_used_space / total_disk_space) * 100.0
    } else {
        0.0
    };

    DiskInfo {
        total_disk_space: format_to_f32(total_disk_space, 2),
        total_used_space: format_to_f32(total_used_space, 2),
        total_free_space: format_to_f32(total_free_space, 2),
        total_usage: format_to_f32(total_usage, 2),
        disks: disk_details,
    }
}

/// 获取Bot状态信息
///
/// 此函数可以获取Bot状态信息，包括CPU使用率、内存使用率、已用内存等
/// # 返回值
///
/// * [BotInfo] - Bot状态信息
///
pub fn get_bot_info() -> BotInfo {
    let mut system = System::new();
    let current_pid = Pid::from_u32(process::id());
    system.refresh_processes(ProcessesToUpdate::Some(&[current_pid]), true);
    let bot = system.process(current_pid);

    let cpu_usage = if let Some(bot) = bot {
        let usage = bot.cpu_usage();
        let decimal_value = Decimal::from_f32(usage).unwrap_or(Decimal::ZERO);
        let rounded = decimal_value.round_dp(2);
        rounded.to_f32().unwrap_or(0.0)
    } else {
        0.0
    };
    let used_memory = if let Some(bot) = bot {
        bot.memory() as f32 / 1024.0 / 1024.0
    } else {
        0.0
    };

    let memory_usage = format_to_f32(used_memory, 2);
    let used_memory_formatted = format_to_f32(used_memory, 2);

    BotInfo {
        cpu_usage,
        memory_usage,
        used_memory: used_memory_formatted,
    }
}
