use puniyu_system_info::ProcessInfo;
use std::time::Duration;


/// 获取应用运行时间
///
/// # 返回
///
/// * `u64` - 应用运行时间，时间戳，单位秒
pub fn uptime() -> u64 {
	ProcessInfo::default().run_time
}

pub(crate) fn format_duration(duration: Duration) -> String {
	let minutes = duration.as_secs() / 60;
	let seconds = duration.as_secs() % 60;
	let milliseconds = duration.subsec_millis();

	let mut parts = Vec::new();

	if minutes > 0 {
		parts.push(format!("{}分", minutes));
	}

	if seconds > 0 {
		parts.push(format!("{}秒", seconds));
	}

	if milliseconds > 0 {
		parts.push(format!("{}毫秒", milliseconds));
	}

	if parts.is_empty() { "0秒".to_string() } else { parts.join("") }
}
