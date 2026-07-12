use bytes::Bytes;
use puniyu_account::AccountInfo;

#[test]
fn test_account_info_creation() {
	let account = AccountInfo {
		id: "123456789".into(),
		name: "MyBot".into(),
		avatar: Bytes::from("image data"),
	};

	assert_eq!(account.id, "123456789");
	assert_eq!(account.name, "MyBot");
	assert_eq!(account.avatar, Bytes::from("image data"));
}

#[test]
fn test_account_info_default() {
	let account = AccountInfo::default();

	assert_eq!(account.id, "");
	assert_eq!(account.name, "");
	assert_eq!(account.avatar, Bytes::new());
}

#[test]
fn test_account_info_clone() {
	let account1 = AccountInfo {
		id: "123456789".into(),
		name: "MyBot".into(),
		avatar: Bytes::from("image data"),
	};
	let account2 = account1.clone();

	assert_eq!(account1, account2);
}

#[test]
fn test_account_info_equality() {
	let account1 = AccountInfo {
		id: "123456789".into(),
		name: "MyBot".into(),
		avatar: Bytes::from("image data"),
	};
	let account2 = AccountInfo {
		id: "123456789".into(),
		name: "MyBot".into(),
		avatar: Bytes::from("image data"),
	};
	let account3 = AccountInfo {
		id: "987654321".into(),
		name: "OtherBot".into(),
		avatar: Bytes::from("other data"),
	};

	assert_eq!(account1, account2);
	assert_ne!(account1, account3);
}

#[test]
fn test_account_info_debug() {
	let account = AccountInfo {
		id: "123456789".into(),
		name: "MyBot".into(),
		avatar: Bytes::from("image data"),
	};
	let debug_str = format!("{:?}", account);

	assert!(debug_str.contains("AccountInfo"));
	assert!(debug_str.contains("123456789"));
	assert!(debug_str.contains("MyBot"));
}

#[test]
fn test_account_info_empty_fields() {
	let account = AccountInfo { id: "".into(), name: "".into(), avatar: Bytes::new() };

	assert_eq!(account.id, "");
	assert_eq!(account.name, "");
	assert_eq!(account.avatar, Bytes::new());
}

#[test]
fn test_account_info_unicode_name() {
	let account = AccountInfo {
		id: "123456789".into(),
		name: "我的机器人".into(),
		avatar: Bytes::from("image data"),
	};

	assert_eq!(account.name, "我的机器人");
}

#[test]
fn test_account_info_special_characters() {
	let account = AccountInfo {
		id: "user@123".into(),
		name: "Bot #1".into(),
		avatar: Bytes::from("binary\x00data"),
	};

	assert_eq!(account.id, "user@123");
	assert_eq!(account.name, "Bot #1");
	assert!(account.avatar.contains(&b'\x00'));
}

#[test]
fn test_account_info_long_id() {
	let account = AccountInfo {
		id: "12345678901234567890".into(),
		name: "MyBot".into(),
		avatar: Bytes::from("image data"),
	};

	assert_eq!(account.id.len(), 20);
}

#[test]
fn test_account_info_serialization() {
	let account = AccountInfo {
		id: "123456789".into(),
		name: "MyBot".into(),
		avatar: Bytes::from("image data"),
	};

	let json = serde_json::to_string(&account).unwrap();
	let deserialized: AccountInfo = serde_json::from_str(&json).unwrap();

	assert_eq!(account, deserialized);
}
