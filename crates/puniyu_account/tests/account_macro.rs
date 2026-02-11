use puniyu_account::account_info;

#[test]
fn test_account_info_macro_full() {
	let account = account_info!(
		uin: "123456789",
		name: "MyBot",
		avatar: "https://example.com/avatar.jpg",
	);

	assert_eq!(account.uin, "123456789");
	assert_eq!(account.name, "MyBot");
	assert_eq!(account.avatar, "https://example.com/avatar.jpg");
}

#[test]
fn test_account_info_macro_unicode() {
	let account = account_info!(
		uin: "123456789",
		name: "我的机器人",
		avatar: "https://example.com/avatar.jpg",
	);

	assert_eq!(account.name, "我的机器人");
}

#[test]
fn test_account_info_macro_trailing_comma() {
	let account = account_info!(
		uin: "123456789",
		name: "MyBot",
		avatar: "https://example.com/avatar.jpg",
	);

	assert_eq!(account.uin, "123456789");
}

#[test]
fn test_account_info_macro_no_trailing_comma() {
	let account = account_info!(
		uin: "123456789",
		name: "MyBot",
		avatar: "https://example.com/avatar.jpg"
	);

	assert_eq!(account.uin, "123456789");
}

#[test]
fn test_account_info_macro_string_conversion() {
	let uin = String::from("123456789");
	let name = String::from("MyBot");
	let avatar = String::from("https://example.com/avatar.jpg");

	let account = account_info!(
		uin: uin,
		name: name,
		avatar: avatar,
	);

	assert_eq!(account.uin, "123456789");
	assert_eq!(account.name, "MyBot");
	assert_eq!(account.avatar, "https://example.com/avatar.jpg");
}

#[test]
fn test_account_info_macro_str_slice() {
	let account = account_info!(
		uin: "123456789",
		name: "MyBot",
		avatar: "https://example.com/avatar.jpg",
	);

	assert_eq!(account.uin, "123456789");
	assert_eq!(account.name, "MyBot");
	assert_eq!(account.avatar, "https://example.com/avatar.jpg");
}

#[test]
fn test_account_info_macro_empty_strings() {
	let account = account_info!(
		uin: "",
		name: "",
		avatar: "",
	);

	assert_eq!(account.uin, "");
	assert_eq!(account.name, "");
	assert_eq!(account.avatar, "");
}

#[test]
fn test_account_info_macro_special_characters() {
	let account = account_info!(
		uin: "user@123",
		name: "Bot #1",
		avatar: "https://example.com/avatar?size=100",
	);

	assert_eq!(account.uin, "user@123");
	assert_eq!(account.name, "Bot #1");
	assert!(account.avatar.contains("?size=100"));
}

#[test]
fn test_account_info_macro_long_values() {
	let account = account_info!(
		uin: "12345678901234567890",
		name: "MyVeryLongBotNameWithManyCharacters",
		avatar: "https://example.com/very/long/path/to/avatar/image.jpg",
	);

	assert_eq!(account.uin.len(), 20);
	assert!(account.name.len() > 20);
	assert!(account.avatar.len() > 30);
}

#[test]
fn test_account_info_macro_positional() {
	let account = account_info!("123456789", "MyBot", "https://example.com/avatar.jpg");

	assert_eq!(account.uin, "123456789");
	assert_eq!(account.name, "MyBot");
	assert_eq!(account.avatar, "https://example.com/avatar.jpg");
}

#[test]
fn test_account_info_macro_positional_empty() {
	let account = account_info!("123456789", "MyBot", "");

	assert_eq!(account.uin, "123456789");
	assert_eq!(account.name, "MyBot");
	assert_eq!(account.avatar, "");
}

#[test]
fn test_account_info_macro_positional_unicode() {
	let account = account_info!("123456789", "我的机器人", "https://example.com/avatar.jpg");

	assert_eq!(account.name, "我的机器人");
}

#[test]
fn test_account_info_macro_positional_string() {
	let uin = String::from("123456789");
	let name = String::from("MyBot");
	let avatar = String::from("https://example.com/avatar.jpg");

	let account = account_info!(uin, name, avatar);

	assert_eq!(account.uin, "123456789");
	assert_eq!(account.name, "MyBot");
	assert_eq!(account.avatar, "https://example.com/avatar.jpg");
}
