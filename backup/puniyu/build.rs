#![allow(clippy::unwrap_used)]

use vergen_git2::{Build, Emitter, Git2};

fn main() {
	println!("cargo:rerun-if-changed=build.rs");
	println!("cargo:rerun-if-changed=../.git/HEAD");
	let build = Build::all_build();
	let git = Git2::all_git();
	Emitter::default()
		.add_instructions(&build)
		.unwrap()
		.add_instructions(&git)
		.unwrap()
		.emit()
		.unwrap();
}
