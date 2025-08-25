fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let plugin_name = env!("CARGO_PKG_NAME"); // 插件名称
    println!("cargo:rustc-env=PLUGIN_NAME={}", plugin_name);
    let plugin_version = env!("CARGO_PKG_VERSION"); // 插件版本
    println!("cargo:rustc-env=PLUGIN_VERSION={}", plugin_version);
    let plugin_author = env!("CARGO_PKG_AUTHORS"); // 插件作者
    println!("cargo:rustc-env=PLUGIN_AUTHOR={}", plugin_author);

    let plugin_rustc_version = rustc_version::version().unwrap(); // rustc版本
    println!(
        "cargo:rustc-env=PLUGIN_RUSTC_VERSION={}",
        plugin_rustc_version
    );
}
