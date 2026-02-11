use puniyu_config::{bot::BotConfig, friend::FriendConfig, group::GroupConfig};

#[test]
fn test_bot_config_inheritance() {
	let toml_str = r#"
        [global]
        cd = 0
        mode = 0
        alias = ["bot"]

        [bot.bot_001]
        cd = 5

        [bot.bot_002]
        mode = 1
        alias = ["helper"]
    "#;

	let config: BotConfig = toml::from_str(toml_str).unwrap();

	// bot_001 should inherit mode and alias from global
	let bot_001 = config.bot("bot_001");
	assert_eq!(bot_001.cd(), 5);
	assert_eq!(bot_001.alias(), vec!["bot"]);

	// bot_002 should inherit cd from global
	let bot_002 = config.bot("bot_002");
	assert_eq!(bot_002.cd(), 0);
	assert_eq!(bot_002.alias(), vec!["helper"]);

	// non-existent bot should use global config
	let bot_003 = config.bot("bot_003");
	assert_eq!(bot_003.cd(), 0);
	assert_eq!(bot_003.alias(), vec!["bot"]);
}

#[test]
fn test_group_config_inheritance() {
	let toml_str = r#"
        [global]
        cd = 0
        user_cd = 0
        mode = 0
        alias = []

        [group.group_123]
        cd = 10

        [group.group_456]
        cd = 10
        user_cd = 5
    "#;

	let config: GroupConfig = toml::from_str(toml_str).unwrap();

	// group_123 should inherit user_cd, mode, and alias from global
	let group_123 = config.group("group_123");
	assert_eq!(group_123.cd(), 10);
	assert_eq!(group_123.user_cd(), 0);
	assert_eq!(group_123.alias(), Vec::<String>::new());

	// group_456 should inherit mode and alias from global
	let group_456 = config.group("group_456");
	assert_eq!(group_456.cd(), 10);
	assert_eq!(group_456.user_cd(), 5);

	// non-existent group should use global config
	let group_789 = config.group("group_789");
	assert_eq!(group_789.cd(), 0);
	assert_eq!(group_789.user_cd(), 0);
}

#[test]
fn test_friend_config_inheritance() {
	let toml_str = r#"
        [global]
        cd = 0
        mode = 0
        alias = ["bot"]

        [friend.user_123]
        cd = 5

        [friend.user_456]
        mode = 1
    "#;

	let config: FriendConfig = toml::from_str(toml_str).unwrap();

	// user_123 should inherit mode and alias from global
	let user_123 = config.friend("user_123");
	assert_eq!(user_123.cd(), 5);
	assert_eq!(user_123.alias(), vec!["bot"]);

	// user_456 should inherit cd and alias from global
	let user_456 = config.friend("user_456");
	assert_eq!(user_456.cd(), 0);

	// non-existent friend should use global config
	let user_789 = config.friend("user_789");
	assert_eq!(user_789.cd(), 0);
	assert_eq!(user_789.alias(), vec!["bot"]);
}

#[test]
fn test_partial_override() {
	let toml_str = r#"
        [global]
        cd = 10
        mode = 0
        alias = ["bot", "助手"]

        [bot.bot_001]
        alias = ["custom"]
    "#;

	let config: BotConfig = toml::from_str(toml_str).unwrap();

	let bot_001 = config.bot("bot_001");
	// Should inherit cd and mode from global
	assert_eq!(bot_001.cd(), 10);
	// Should override alias
	assert_eq!(bot_001.alias(), vec!["custom"]);
}

#[test]
fn test_empty_specific_config() {
	let toml_str = r#"
        [global]
        cd = 5
        mode = 1
        alias = ["bot"]

        [bot.bot_001]
    "#;

	let config: BotConfig = toml::from_str(toml_str).unwrap();

	// bot_001 has no specific settings, should use all global values
	let bot_001 = config.bot("bot_001");
	assert_eq!(bot_001.cd(), 5);
	assert_eq!(bot_001.alias(), vec!["bot"]);
}

#[test]
fn test_list_with_inheritance() {
	let toml_str = r#"
        [global]
        cd = 0
        mode = 0
        alias = ["bot"]

        [bot.bot_001]
        cd = 5

        [bot.bot_002]
        cd = 10
    "#;

	let config: BotConfig = toml::from_str(toml_str).unwrap();

	let list = config.list();
	assert_eq!(list.len(), 2);

	// All bots in list should have inherited values
	let bot_001 = list.get("bot_001").unwrap();
	assert_eq!(bot_001.cd(), 5);
	assert_eq!(bot_001.alias(), vec!["bot"]);

	let bot_002 = list.get("bot_002").unwrap();
	assert_eq!(bot_002.cd(), 10);
	assert_eq!(bot_002.alias(), vec!["bot"]);
}
