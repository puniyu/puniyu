fn main() {
	let bin_path = protoc_bin_vendored::protoc_bin_path().unwrap();
	let mut prost_build = prost_build::Config::new();
	prost_build.protoc_executable(&bin_path);
	prost_build
		.compile_protos(
			&["protos/*"],
			&["protos/"],
		)
		.unwrap();
}
