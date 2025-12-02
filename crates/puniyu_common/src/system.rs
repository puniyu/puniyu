pub use puniyu_system_info::{
	CpuInfo, DiskInfo, GpuInfo, HostInfo, MemoryInfo, ProcessInfo, SystemInfo as System,
};

#[derive(Debug, Clone, Default)]
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


impl SystemInfo {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn cpu() -> CpuInfo {
		CpuInfo::new()
	}

	pub fn memory() -> MemoryInfo {
		MemoryInfo::new()
	}

	pub fn disk() -> DiskInfo {
		DiskInfo::new()
	}

	pub fn host() -> HostInfo {
		HostInfo::new()
	}

	pub fn bot() -> BotStatusInfo {
		BotStatusInfo::default()
	}

	pub fn process_with_pid(pid: u32) -> ProcessInfo {
		System::process_with_pid(pid)
	}

	pub fn gpu() -> Option<GpuInfo> {
		GpuInfo::new()
	}
}

#[derive(Debug, Clone)]
pub struct BotStatusInfo {
	/// 进程PID
	pub pid: u32,
	/// CPU使用率
	pub cpu_usage: Option<f32>,
	/// 内存使用率
	pub memory_usage: Option<f32>,
	/// 已用内存(单位: MB)
	pub used_memory: f64,
	/// 运行时间
	pub run_time: u64,
}

impl Default for BotStatusInfo {
	fn default() -> Self {
		let process_info = ProcessInfo::default();
		Self {
			pid: process_info.pid.as_u32(),
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
