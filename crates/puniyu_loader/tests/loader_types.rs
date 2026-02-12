use puniyu_loader::LoaderId;

#[test]
fn test_loader_id_from_index() {
	let id = LoaderId::from(42u64);
	match id {
		LoaderId::Index(idx) => assert_eq!(idx, 42),
		_ => panic!("Expected Index variant"),
	}
}

#[test]
fn test_loader_id_from_name() {
	let id = LoaderId::from("test_loader");
	match id {
		LoaderId::Name(name) => assert_eq!(name, "test_loader"),
		_ => panic!("Expected Name variant"),
	}
}

#[test]
fn test_loader_id_index_zero() {
	let id = LoaderId::from(0u64);
	match id {
		LoaderId::Index(idx) => assert_eq!(idx, 0),
		_ => panic!("Expected Index variant"),
	}
}

#[test]
fn test_loader_id_large_index() {
	let id = LoaderId::from(u64::MAX);
	match id {
		LoaderId::Index(idx) => assert_eq!(idx, u64::MAX),
		_ => panic!("Expected Index variant"),
	}
}

#[test]
fn test_loader_id_empty_name() {
	let id = LoaderId::from("");
	match id {
		LoaderId::Name(name) => assert_eq!(name, ""),
		_ => panic!("Expected Name variant"),
	}
}

#[test]
fn test_loader_id_unicode_name() {
	let id = LoaderId::from("加载器");
	match id {
		LoaderId::Name(name) => assert_eq!(name, "加载器"),
		_ => panic!("Expected Name variant"),
	}
}

#[test]
fn test_loader_id_special_characters() {
	let id = LoaderId::from("loader@123");
	match id {
		LoaderId::Name(name) => assert_eq!(name, "loader@123"),
		_ => panic!("Expected Name variant"),
	}
}

#[test]
fn test_loader_id_clone() {
	let id1 = LoaderId::from(42u64);
	let id2 = id1.clone();
	assert_eq!(id1, id2);
}

#[test]
fn test_loader_id_equality() {
	let id1 = LoaderId::from(42u64);
	let id2 = LoaderId::from(42u64);
	let id3 = LoaderId::from(43u64);

	assert_eq!(id1, id2);
	assert_ne!(id1, id3);
}

#[test]
fn test_loader_id_name_equality() {
	let id1 = LoaderId::from("loader");
	let id2 = LoaderId::from("loader");
	let id3 = LoaderId::from("other");

	assert_eq!(id1, id2);
	assert_ne!(id1, id3);
}

#[test]
fn test_loader_id_debug() {
	let id1 = LoaderId::from(42u64);
	let debug_str = format!("{:?}", id1);
	assert!(debug_str.contains("Index"));
	assert!(debug_str.contains("42"));

	let id2 = LoaderId::from("test");
	let debug_str = format!("{:?}", id2);
	assert!(debug_str.contains("Name"));
	assert!(debug_str.contains("test"));
}
