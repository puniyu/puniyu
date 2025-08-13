use puniyu_system_info::get_process_info;
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
    let process_info = get_process_info();
    process_info.run_time
}
