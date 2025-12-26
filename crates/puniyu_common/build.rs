use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let channel = match env::var("VERSION_CHANNEL") {
        Ok(val) if val.to_lowercase() == "stable" => "stable",
        _ => "nightly",
    };
    println!("cargo:rustc-env=VERSION_CHANNEL={}", channel);
    println!("cargo:rerun-if-env-changed=VERSION_CHANNEL");
}