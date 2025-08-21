use puniyu_registry::plugin;

#[plugin]
pub async fn test() {
    log::info!(
        "{} v{} 初始化完成",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
}
