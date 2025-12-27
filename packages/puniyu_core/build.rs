use std::env;
use std::fs;
use std::path::Path;

fn main() {
	let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().parent().unwrap();
	let logo_path = workspace_dir.join("logo.png");

	if logo_path.exists() {
		let out_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
		let assets_dir = Path::new(&out_dir).join("assets");
		fs::create_dir_all(&assets_dir).unwrap();

		let target_path = assets_dir.join("logo.png");
		fs::copy(&logo_path, &target_path).unwrap();
		println!("cargo:rerun-if-changed={}", logo_path.display());
	}
}
