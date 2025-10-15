use crate::system::get_bot_info;
use std::{thread, time::Duration};

/// 等待一段时间
///
/// # 参数
///
/// * `ms` - 等待的时长，单位为毫秒
pub fn sleep(ms: u64) {
	thread::sleep(Duration::from_millis(ms));
}

/// 获取系统运行时间
///
/// # 返回
///
/// * `String` - 系统运行时间
pub fn uptime() -> String {
	let time = get_bot_info().run_time;
	println!("uptime: {}", time);
	format_duration(Duration::from_secs(time))
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
