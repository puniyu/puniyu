use glob::glob;
use protoc_bin_vendored::protoc_bin_path;

fn main() {
	let bin_path = protoc_bin_path().expect("protoc executable not found");
	let mut prost_build = prost_build::Config::new();
	prost_build.protoc_executable(&bin_path);
	prost_build.bytes(["."]);
	prost_build.boxed("puniyu.event.Event.event.message_event");
	let proto_files = glob("protos/**/*.proto")
		.expect("Failed to read glob pattern")
		.filter_map(|entry| entry.ok())
		.map(|path| path.to_string_lossy().to_string())
		.collect::<Vec<_>>();
	prost_build.compile_protos(&proto_files, &["protos/"]).expect("Failed to compile protos");
}
