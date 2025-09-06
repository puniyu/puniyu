use crate::config::init_config;
use crate::logger::log_init;

pub struct Bot;

impl Bot {
    /// TODO: 插件绑定， 适配器绑定
    pub fn new() -> Self {
        // 初始化配置文件
        init_config();
        // 初始化日志系统
        log_init();
        // TODO: 事件初始化，多线程定时器初始化
        Self
    }
}
