pub use puniyu_api::path::*;

pub mod plugin {
    use std::path::PathBuf;
    use puniyu_path::plugin as puniyu_path;
    #[inline]
    pub fn config_dir(name: &str) -> PathBuf {
        puniyu_path::CONFIG_DIR.join(name)
    }

    #[inline]
    pub fn data_dir(name: &str) -> PathBuf {
        puniyu_path::DATA_DIR.join(name)
    }

    #[inline]
    pub fn resource_dir(name: &str) -> PathBuf {
        puniyu_path::RESOURCE_DIR.join(name)
    }

    #[inline]
    pub fn temp_dir(name: &str) -> PathBuf {
        puniyu_path::TEMP_DIR.join(name)
    }

}