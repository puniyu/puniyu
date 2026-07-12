use bytes::Bytes;
use puniyu_account::AccountInfo;

#[test]
fn test_account_info_builder_full() {
	let account = AccountInfo::builder()
		.id("123456789")
		.name("MyBot")
		.avatar(Bytes::from_static(b"image data"))
		.build();

	assert_eq!(
		account,
		AccountInfo {
			id: "123456789".into(),
			name: "MyBot".into(),
			avatar: Bytes::from_static(b"image data"),
		}
	);
}

#[test]
fn test_account_info_builder_setter_into() {
	let account = AccountInfo::builder()
		.id(String::from("123456789"))
		.name(String::from("MyBot"))
		.avatar(Vec::from(&b"avatar"[..]))
		.build();

	assert_eq!(account.id, "123456789");
	assert_eq!(account.name, "MyBot");
	assert_eq!(account.avatar, Bytes::from_static(b"avatar"));
}
