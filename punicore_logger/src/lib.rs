use owo_colors::OwoColorize;
use chrono_tz::Asia::Shanghai;
use std::fmt;
use tracing::Subscriber;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{filter::LevelFilter, layer::SubscriberExt, Layer, registry::LookupSpan, fmt::{FormatEvent, FormatFields}};

pub struct LoggerOptions {
    /// 日志等级
    pub level: LevelFilter,
    /// 是否启用文件日志记录
    pub enable_file_logging: bool,
    /// 日志文件保存路径
    pub log_directory: Option<String>,
    /// 日志文件保留天数
    pub retention_days: Option<u32>,
}

impl LoggerOptions {
    /// 创建新的日志配置选项
    pub fn new(level: LevelFilter) -> Self {
        Self {
            level,
            enable_file_logging: false,
            log_directory: None,
            retention_days: None,
        }
    }
    /// 设置是否启用文件日志记录
    pub fn with_file_logging(mut self, enable: bool) -> Self {
        self.enable_file_logging = enable;
        self
    }

    /// 设置日志文件保存目录
    pub fn with_log_directory(mut self, directory: String) -> Self {
        self.log_directory = Some(directory);
        self
    }
    /// 设置日志文件保留天数
    pub fn with_retention_days(mut self, days: u32) -> Self {
        self.retention_days = Some(days);
        self
    }
}

struct PrefixFormatter {
    show_target: bool,
    color: bool,
}

impl<S, N> FormatEvent<S, N> for PrefixFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &tracing_subscriber::fmt::FmtContext<'_, S, N>,
        mut writer: tracing_subscriber::fmt::format::Writer<'_>,
        event: &tracing::Event<'_>,
    ) -> fmt::Result {
        let prefix = if self.color {
            "[Puni]".magenta().to_string()
        } else {
            "[Puni]".to_string()
        };
        write!(writer, "{} ", prefix)?;

        let local_time = chrono::Local::now();
        let shanghai_time = local_time.with_timezone(&Shanghai);
        let formatted_time = shanghai_time.format("%H:%M:%S%.3f");
        write!(writer, "[{}] ", formatted_time)?;

        let logger_level = event.metadata().level();
        if self.color {
            let colored_level = match *logger_level {
                tracing::Level::ERROR => logger_level.red().to_string(),
                tracing::Level::WARN => logger_level.yellow().to_string(),
                tracing::Level::INFO => logger_level.green().to_string(),
                tracing::Level::DEBUG => logger_level.blue().to_string(),
                tracing::Level::TRACE => logger_level.magenta().to_string(),
            };
            write!(writer, "[{: <17}] ", colored_level)?;
        } else {
            write!(writer, "[{: <7}] ", logger_level)?;
        }


        if self.show_target {
            if self.color {
                write!(writer, "[{}] ", event.metadata().target().purple())?;
            } else {
                write!(writer, "[{}] ", event.metadata().target())?;
            }
        }


        ctx.format_fields(writer.by_ref(), event)?;
        writeln!(writer)
    }
}

pub fn log_init(options: Option<LoggerOptions>) {
    let options = options.unwrap_or_else(|| LoggerOptions::new(LevelFilter::INFO));

    let logger_level = match options.level {
        LevelFilter::OFF => LevelFilter::OFF,
        LevelFilter::ERROR => LevelFilter::ERROR,
        LevelFilter::WARN => LevelFilter::WARN,
        LevelFilter::INFO => LevelFilter::INFO,
        LevelFilter::DEBUG => LevelFilter::DEBUG,
        LevelFilter::TRACE => LevelFilter::TRACE,
    };

    let show_target = matches!(logger_level, LevelFilter::DEBUG | LevelFilter::TRACE);

    // 创建控制台日志层
    let console_subscriber = tracing_subscriber::fmt::layer()
        .event_format(PrefixFormatter { show_target, color: true })
        .with_filter(logger_level);

    let mut layers = vec![console_subscriber.boxed()];

    if options.enable_file_logging {
        let log_dir = options.log_directory.unwrap_or_else(|| "logs".to_string());
        let _ = std::fs::create_dir_all(&log_dir);
        let file_appender = RollingFileAppender::builder()
            .rotation(Rotation::DAILY)
            .filename_prefix("puni")
            .filename_suffix("log")
            .max_log_files(options.retention_days.unwrap_or(7) as usize)
            .build(&log_dir)
            .unwrap();


        let file_subscriber = tracing_subscriber::fmt::layer()
            .event_format(PrefixFormatter { show_target: true, color: false })
            .with_writer(file_appender)
            .with_ansi(false)
            .with_filter(logger_level);

        layers.push(file_subscriber.boxed());
    }

    let subscriber = tracing_subscriber::registry()
        .with(layers);

    tracing::subscriber::set_global_default(subscriber).unwrap();
    tracing_log::LogTracer::init().unwrap();
}
