#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        {
            use ::puniyu_logger::owo_colors::OwoColorize;
            let prefix = "Server".fg_rgb::<132,112,255>();
            ::puniyu_logger::info!("[{}] {}", prefix, format!($($arg)*))
        }
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        {
            use ::puniyu_logger::owo_colors::OwoColorize;
            let prefix = "Server".fg_rgb::<132,112,255>();
            ::puniyu_logger::warn!("[{}] {}", prefix, format!($($arg)*))
        }
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        {
            use ::puniyu_logger::owo_colors::OwoColorize;
            let prefix = "Server".fg_rgb::<132,112,255>();
            ::puniyu_logger::error!("[{}] {}", prefix, format!($($arg)*))
        }
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        {
            use ::puniyu_logger::owo_colors::OwoColorize;
            let prefix = "Server".fg_rgb::<132,112,255>();
            ::puniyu_logger::debug!("[{}] {}", prefix, format!($($arg)*))
        }
    };
}
