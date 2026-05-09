#![allow(clippy::unwrap_used)]

use vergen_git2::{BuildBuilder, Emitter, Git2Builder};
use cargo_metadata::MetadataCommand;

fn main() {
	println!("cargo:rerun-if-changed=build.rs");
	println!("cargo:rerun-if-changed=../.git/HEAD");

	let workspace_dir = MetadataCommand::new()
		.current_dir(env!("CARGO_MANIFEST_DIR"))
		.exec()
		.unwrap()
		.workspace_root;
	println!("cargo:rustc-env=WORKSPACE_DIR={workspace_dir}");
	let build = BuildBuilder::all_build().unwrap();
	let git = Git2Builder::all_git().unwrap();
	Emitter::default()
		.add_instructions(&build)
		.unwrap()
		.add_instructions(&git)
		.unwrap()
		.emit()
		.unwrap();
}
