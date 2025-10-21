pub use puniyu_system_info::{
	CpuInfo, DiskInfo, GpuInfo, HostInfo, MemoryInfo, Pid, ProcessInfo, SystemInfo as System,
};

#[derive(Debug, Clone)]
/// 系统信息
pub struct SystemInfo {
	/// CPU信息
	pub cpu: CpuInfo,
	/// 内存信息
	pub memory: MemoryInfo,
	/// 硬盘信息
	pub disk: DiskInfo,
	/// 主机信息
	pub host: HostInfo,
	/// GPU信息
	pub gpu: Option<GpuInfo>,
	/// Bot状态信息
	pub bot: BotStatusInfo,
}

/// 获取系统信息
///
/// 此函数可以获取系统信息，包括CPU、内存、磁盘、Bot信息等
/// # 返回值
///
/// * [SystemInfo] - 系统信息
///
impl Default for SystemInfo {
	fn default() -> Self {
		let cpu = System::cpu();
		let host = System::host();
		let memory = System::memory();
		let disk = System::disk();
		let bot = BotStatusInfo::default();
		let gpu = System::gpu();
		SystemInfo { cpu, memory, disk, host, gpu, bot }
	}
}

impl SystemInfo {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn cpu() -> CpuInfo {
		System::cpu()
	}

	pub fn memory() -> MemoryInfo {
		System::memory()
	}

	pub fn disk() -> DiskInfo {
		System::disk()
	}

	pub fn host() -> HostInfo {
		System::host()
	}

	pub fn bot() -> BotStatusInfo {
		BotStatusInfo::default()
	}

	pub fn process_with_pid(pid: u32) -> ProcessInfo {
		System::process_with_pid(pid)
	}

	pub fn gpu() -> Option<GpuInfo> {
		System::gpu()
	}
}

#[derive(Debug, Clone)]
pub struct BotStatusInfo {
	/// 进程PID
	pub pid: Pid,
	/// CPU使用率
	pub cpu_usage: Option<u8>,
	/// 内存使用率
	pub memory_usage: Option<u8>,
	/// 已用内存(单位: MB)
	pub used_memory: f32,
	/// 运行时间
	pub run_time: u64,
}

impl Default for BotStatusInfo {
	fn default() -> Self {
		let process_info = System::process();
		Self {
			pid: process_info.pid,
			cpu_usage: process_info.cpu_usage,
			memory_usage: process_info.memory_usage,
			used_memory: process_info.used_memory,
			run_time: process_info.run_time,
		}
	}
}
impl BotStatusInfo {
	pub fn new() -> Self {
		Self::default()
	}
}
