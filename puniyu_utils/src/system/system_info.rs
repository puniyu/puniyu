use std::{process};
use puniyu_system_info::{CpuInfo, HostInfo, DiskInfo, MemoryInfo, get_disk_info, get_memory_info, get_host_info, get_cpu_info, get_procss_info};

#[derive(Debug)]
pub struct SystemInfo {
    /// 系统信息
    pub cpu: CpuInfo,
    /// 内存信息
    pub memory: MemoryInfo,
    /// 硬盘信息
    pub disk: DiskInfo,
    /// 主机信息
    pub host: HostInfo,
    /// Bot状态信息
    pub bot: BotStatusInfo,
}

#[derive(Debug)]
pub struct BotStatusInfo {
    /// 进程PID
    pub pid: u32,
    /// CPU使用率
    pub cpu_usage: Option<u8>,
    /// 内存使用率
    pub memory_usage: Option<u8>,
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
    let host = get_host_info();
    let memory = get_memory_info();
    let disk = get_disk_info();
    let bot = get_bot_info();
    SystemInfo {
        cpu,
        memory,
        disk,
        host,
        bot,
    }
}

/// 获取进程PID
///
/// 此函数可以获取当前进程的PID
/// # 返回值
///
/// * [u32] - 进程PID
///
pub fn get_process_pid() -> u32 {
    process::id()
}

/// 获取Bot状态信息
///
/// 此函数可以获取Bot状态信息，包括CPU使用率、内存使用率、已用内存等
/// # 返回值
///
/// * [BotStatusInfo] - Bot状态信息
///
pub fn get_bot_info() -> BotStatusInfo {
    let process = get_procss_info();

    BotStatusInfo {
        pid: process.pid.as_u32(),
        cpu_usage: process.cpu_usage,
        memory_usage: process.memory_usage,
        used_memory: process.used_memory,
    }
}
