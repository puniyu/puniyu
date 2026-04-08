#![allow(unused_macros, unused_imports)]

macro_rules! server_info {
    ($($arg:tt)*) => {
        {
            #[cfg(feature = "cli")]
            {
                ::log::info!("{}", format!($($arg)*))
            }
            #[cfg(not(feature = "cli"))]
            {
                use ::puniyu_logger::owo_colors::OwoColorize;
                let prefix = "Server".fg_rgb::<132,112,255>();
                ::log::info!("[{}] {}", prefix, format!($($arg)*))
            }
        }
    };
}

macro_rules! server_warn {
    ($($arg:tt)*) => {
        {
            #[cfg(feature = "cli")]
            {
                ::log::warn!("{}", format!($($arg)*))
            }
            #[cfg(not(feature = "cli"))]
            {
                use ::puniyu_logger::owo_colors::OwoColorize;
                let prefix = "Server".fg_rgb::<132,112,255>();
                ::log::warn!("[{}] {}", prefix, format!($($arg)*))
            }
        }
    };
}

macro_rules! server_error {
    ($($arg:tt)*) => {
        {
            #[cfg(feature = "cli")]
            {
                ::log::error!("{}", format!($($arg)*))
            }
            #[cfg(not(feature = "cli"))]
            {
                use ::puniyu_logger::owo_colors::OwoColorize;
                let prefix = "Server".fg_rgb::<132,112,255>();
                ::log::error!("[{}] {}", prefix, format!($($arg)*))
            }
        }
    };
}

macro_rules! server_debug {
    ($($arg:tt)*) => {
        {
            #[cfg(feature = "cli")]
            {
                ::log::debug!("{}", format!($($arg)*))
            }
            #[cfg(not(feature = "cli"))]
            {
                use ::puniyu_logger::owo_colors::OwoColorize;
                let prefix = "Server".fg_rgb::<132,112,255>();
                ::log::debug!("[{}] {}", prefix, format!($($arg)*))
            }
        }
    };
}
pub(crate) use {
	server_debug as debug, server_error as error, server_info as info, server_warn as warn,
};
