pub use puniyu_system_info::{CpuInfo, DiskInfo, HostInfo, MemoryInfo, SystemInfo as System};
use std::process;

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
	/// 运行时间
	pub run_time: u64,
}

/// 获取系统信息
///
/// 此函数可以获取系统信息，包括CPU、内存、磁盘、Bot信息等
/// # 返回值
///
/// * [SystemInfo] - 系统信息
///
pub fn get_system_info() -> SystemInfo {
	let cpu = System::cpu();
	let host = System::host();
	let memory = System::memory();
	let disk = System::disk();
	let bot = get_bot_info();
	SystemInfo { cpu, memory, disk, host, bot }
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
	let process_info = System::process();
	BotStatusInfo {
		pid: process_info.pid.as_u32(),
		cpu_usage: process_info.cpu_usage,
		memory_usage: process_info.memory_usage,
		used_memory: process_info.used_memory,
		run_time: process_info.run_time,
	}
}
