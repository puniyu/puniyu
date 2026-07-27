// pub mod adapter;
// mod app;
// pub mod runtime;

// use std::{path::PathBuf, sync::LazyLock};

// pub use adapter::AdapterPlugin;
// pub use app::{App, AppBuilder};
// use semver::Version;

// pub const NAME: &str = env!("CARGO_PKG_NAME");
// pub const VERSION: Version = puniyu_version::VERSION;
// #[allow(clippy::unwrap_used)]
// pub static PATH: LazyLock<PathBuf> = LazyLock::new(|| std::env::current_dir().unwrap().join(NAME));