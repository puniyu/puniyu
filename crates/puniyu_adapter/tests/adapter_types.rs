use puniyu_adapter::types::AdapterId;

#[test]
fn test_adapter_id_from_u64() {
	let id: AdapterId = 123u64.into();
	match id {
		AdapterId::Index(index) => assert_eq!(index, 123),
		_ => panic!("Expected AdapterId::Index"),
	}
}

#[test]
fn test_adapter_id_from_str() {
	let id: AdapterId = "test_adapter".into();
	match id {
		AdapterId::Name(name) => assert_eq!(name, "test_adapter"),
		_ => panic!("Expected AdapterId::Name"),
	}
}

#[test]
fn test_adapter_id_index_variant() {
	let id = AdapterId::Index(456);
	match id {
		AdapterId::Index(index) => assert_eq!(index, 456),
		_ => panic!("Expected AdapterId::Index"),
	}
}

#[test]
fn test_adapter_id_name_variant() {
	let id = AdapterId::Name("my_adapter");
	match id {
		AdapterId::Name(name) => assert_eq!(name, "my_adapter"),
		_ => panic!("Expected AdapterId::Name"),
	}
}

#[test]
fn test_adapter_id_zero_index() {
	let id: AdapterId = 0u64.into();
	match id {
		AdapterId::Index(index) => assert_eq!(index, 0),
		_ => panic!("Expected AdapterId::Index"),
	}
}

#[test]
fn test_adapter_id_empty_name() {
	let id: AdapterId = "".into();
	match id {
		AdapterId::Name(name) => assert_eq!(name, ""),
		_ => panic!("Expected AdapterId::Name"),
	}
}

#[test]
fn test_adapter_id_unicode_name() {
	let id: AdapterId = "适配器_测试".into();
	match id {
		AdapterId::Name(name) => assert_eq!(name, "适配器_测试"),
		_ => panic!("Expected AdapterId::Name"),
	}
}

#[test]
fn test_adapter_id_large_index() {
	let id: AdapterId = u64::MAX.into();
	match id {
		AdapterId::Index(index) => assert_eq!(index, u64::MAX),
		_ => panic!("Expected AdapterId::Index"),
	}
}
