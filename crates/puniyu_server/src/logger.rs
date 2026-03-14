#![allow(unused_macros, unused_imports)]

macro_rules! server_info {
    ($($arg:tt)*) => {
        {
            use ::puniyu_logger::owo_colors::OwoColorize;
            let prefix = "Server".fg_rgb::<132,112,255>();
            ::puniyu_logger::info!("[{}] {}", prefix, format!($($arg)*))
        }
    };
}

macro_rules! server_warn {
    ($($arg:tt)*) => {
        {
            use ::puniyu_logger::owo_colors::OwoColorize;
            let prefix = "Server".fg_rgb::<132,112,255>();
            ::puniyu_logger::warn!("[{}] {}", prefix, format!($($arg)*))
        }
    };
}

macro_rules! server_error {
    ($($arg:tt)*) => {
        {
            use ::puniyu_logger::owo_colors::OwoColorize;
            let prefix = "Server".fg_rgb::<132,112,255>();
            ::puniyu_logger::error!("[{}] {}", prefix, format!($($arg)*))
        }
    };
}

macro_rules! server_debug {
    ($($arg:tt)*) => {
        {
            use ::puniyu_logger::owo_colors::OwoColorize;
            let prefix = "Server".fg_rgb::<132,112,255>();
            ::puniyu_logger::debug!("[{}] {}", prefix, format!($($arg)*))
        }
    };
}
pub(crate) use {
	server_debug as debug, server_error as error, server_info as info, server_warn as warn,
};
