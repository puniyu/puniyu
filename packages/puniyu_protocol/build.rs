use glob::glob;

fn main() {
	let bin_path = protoc_bin_vendored::protoc_bin_path().unwrap();
	let mut prost_build = prost_build::Config::new();
	prost_build.protoc_executable(&bin_path);
	prost_build.bytes(["."]);
	let proto_files = glob("protos/**/*.proto")
		.unwrap()
		.filter_map(|entry| entry.ok())
		.map(|path| path.to_string_lossy().to_string())
		.collect::<Vec<_>>();
	prost_build.compile_protos(&proto_files, &["protos/"]).unwrap();
}
