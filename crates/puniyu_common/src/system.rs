use puniyu_system_info::ProcessInfo;


#[derive(Debug, Clone)]
/// 应用程序状态信息
pub struct SystemInfo {
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

impl Default for SystemInfo {
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
impl SystemInfo {
	pub fn new() -> Self {
		Self::default()
	}
}
