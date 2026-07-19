use puniyu_loader::Loader as _;
use puniyu_loader_builtin::Loader;

#[test]
fn test_builtin_loader_default() {
	let loader = Loader::default();
	assert_eq!(loader.name(), "builtin");
}

#[test]
fn test_builtin_loader_name() {
	let loader = Loader::new();
	assert_eq!(loader.name(), "builtin");
}

#[test]
fn test_builtin_loader_new() {
	let loader = Loader::new();
	assert_eq!(loader.name(), "builtin");
}
