use bytes::Bytes;
use puniyu_account::{AccountInfo, AccountInfoBuilder};

#[test]
fn test_account_info_builder_full() {
	let account = match AccountInfoBuilder::default()
		.uin("123456789")
		.name("MyBot")
		.avatar(Bytes::from_static(b"image data"))
		.build()
	{
		Ok(account) => account,
		Err(err) => panic!("builder should create AccountInfo: {err}"),
	};

	assert_eq!(
		account,
		AccountInfo {
			uin: "123456789".to_string(),
			name: "MyBot".to_string(),
			avatar: Bytes::from_static(b"image data"),
		}
	);
}

#[test]
fn test_account_info_builder_setter_into() {
	let account = match AccountInfoBuilder::default()
		.uin(String::from("123456789"))
		.name(String::from("MyBot"))
		.avatar(Vec::from(&b"avatar"[..]))
		.build()
	{
		Ok(account) => account,
		Err(err) => panic!("builder should accept Into setters: {err}"),
	};

	assert_eq!(account.uin, "123456789");
	assert_eq!(account.name, "MyBot");
	assert_eq!(account.avatar, Bytes::from_static(b"avatar"));
}

#[test]
fn test_account_info_builder_missing_required_field() {
	let result = AccountInfoBuilder::default().uin("123456789").avatar(Bytes::new()).build();

	let err = match result {
		Ok(account) => panic!("builder should reject missing name: {account:?}"),
		Err(err) => err,
	};

	assert!(err.to_string().contains("name"));
}
