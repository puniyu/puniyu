use puniyu_sender::{group_sender, GroupSender, Role, Sender, Sex};

#[test]
fn test_group_sender_full_params() {
    let sender = group_sender!(
        user_id: "20001",
        nick: "Alice",
        sex: Sex::Female,
        age: 20,
        role: Role::Admin,
        card: "管理员",
        level: "L10",
        title: "活跃分子"
    );

    assert_eq!(sender.user_id(), "20001");
    assert_eq!(sender.name(), Some("Alice"));
    assert_eq!(sender.sex(), Sex::Female);
    assert_eq!(sender.age(), 20);
    assert_eq!(sender.role(), Role::Admin);
    assert_eq!(sender.card(), Some("管理员"));
    assert_eq!(sender.level(), Some("L10"));
    assert_eq!(sender.title(), Some("活跃分子"));
}

#[test]
fn test_group_sender_with_role_only() {
    let sender = group_sender!(
        user_id: "20002",
        nick: "Bob",
        sex: Sex::Male,
        age: 25,
        role: Role::Owner
    );

    assert_eq!(sender.user_id(), "20002");
    assert_eq!(sender.name(), Some("Bob"));
    assert_eq!(sender.sex(), Sex::Male);
    assert_eq!(sender.age(), 25);
    assert_eq!(sender.role(), Role::Owner);
    assert_eq!(sender.card(), None);
    assert_eq!(sender.level(), None);
    assert_eq!(sender.title(), None);
}

#[test]
fn test_group_sender_with_card_only() {
    let sender = group_sender!(
        user_id: "20003",
        nick: "Charlie",
        sex: Sex::Unknown,
        age: 30,
        role: Role::Member,
        card: "技术组"
    );

    assert_eq!(sender.user_id(), "20003");
    assert_eq!(sender.card(), Some("技术组"));
    assert_eq!(sender.level(), None);
    assert_eq!(sender.title(), None);
}

#[test]
fn test_group_sender_with_card_and_level() {
    let sender = group_sender!(
        user_id: "20004",
        nick: "David",
        sex: Sex::Male,
        age: 28,
        role: Role::Admin,
        card: "副管",
        level: "L8"
    );

    assert_eq!(sender.user_id(), "20004");
    assert_eq!(sender.card(), Some("副管"));
    assert_eq!(sender.level(), Some("L8"));
    assert_eq!(sender.title(), None);
}

#[test]
fn test_group_sender_positional_params() {
    let sender = group_sender!("20005", "Eve", Sex::Female, 22);

    assert_eq!(sender.user_id(), "20005");
    assert_eq!(sender.name(), Some("Eve"));
    assert_eq!(sender.sex(), Sex::Female);
    assert_eq!(sender.age(), 22);
    assert!(sender.role().is_unknown());
    assert_eq!(sender.card(), None);
}

#[test]
fn test_group_sender_without_age() {
    let sender = group_sender!("20006", "Frank", Sex::Male);

    assert_eq!(sender.user_id(), "20006");
    assert_eq!(sender.name(), Some("Frank"));
    assert_eq!(sender.sex(), Sex::Male);
    assert_eq!(sender.age(), 0);
}

#[test]
fn test_group_sender_without_sex_and_age() {
    let sender = group_sender!("20007", "Grace");

    assert_eq!(sender.user_id(), "20007");
    assert_eq!(sender.name(), Some("Grace"));
    assert!(sender.sex().is_unknown());
    assert_eq!(sender.age(), 0);
}

#[test]
fn test_group_sender_only_user_id() {
    let sender = group_sender!("20008");

    assert_eq!(sender.user_id(), "20008");
    assert_eq!(sender.name(), None);
    assert!(sender.sex().is_unknown());
    assert_eq!(sender.age(), 0);
    assert!(sender.role().is_unknown());
}

#[test]
fn test_group_sender_direct_construction() {
    let sender = GroupSender {
        user_id: "20009".to_string(),
        nick: Some("Henry".to_string()),
        sex: Sex::Male,
        age: 35,
        role: Role::Owner,
        card: Some("群主".to_string()),
        level: Some("MAX".to_string()),
        title: Some("创始人".to_string()),
    };

    assert_eq!(sender.user_id(), "20009");
    assert_eq!(sender.name(), Some("Henry"));
    assert_eq!(sender.role(), Role::Owner);
    assert_eq!(sender.card(), Some("群主"));
    assert_eq!(sender.level(), Some("MAX"));
    assert_eq!(sender.title(), Some("创始人"));
}

#[test]
fn test_group_sender_clone() {
    let sender = group_sender!(
        user_id: "20010",
        nick: "Ivy",
        sex: Sex::Female,
        age: 19,
        role: Role::Admin
    );
    let cloned = sender.clone();

    assert_eq!(sender.user_id(), cloned.user_id());
    assert_eq!(sender.name(), cloned.name());
    assert_eq!(sender.sex(), cloned.sex());
    assert_eq!(sender.age(), cloned.age());
    assert_eq!(sender.role(), cloned.role());
}

#[test]
fn test_role_is_methods() {
    assert!(Role::Owner.is_owner());
    assert!(!Role::Owner.is_admin());
    assert!(!Role::Owner.is_member());
    assert!(!Role::Owner.is_unknown());

    assert!(Role::Admin.is_admin());
    assert!(!Role::Admin.is_owner());
    assert!(!Role::Admin.is_member());
    assert!(!Role::Admin.is_unknown());

    assert!(Role::Member.is_member());
    assert!(!Role::Member.is_owner());
    assert!(!Role::Member.is_admin());
    assert!(!Role::Member.is_unknown());

    assert!(Role::Unknown.is_unknown());
    assert!(!Role::Unknown.is_owner());
    assert!(!Role::Unknown.is_admin());
    assert!(!Role::Unknown.is_member());
}

#[test]
fn test_group_sender_serialization() {
    let sender = group_sender!(
        user_id: "20011",
        nick: "Jack",
        sex: Sex::Male,
        age: 27,
        role: Role::Admin,
        card: "管理",
        level: "L5",
        title: "优秀成员"
    );
    let json = serde_json::to_string(&sender).expect("Failed to serialize");

    assert!(json.contains("20011"));
    assert!(json.contains("Jack"));
    assert!(json.contains("Admin"));
}

#[test]
fn test_group_sender_deserialization() {
    let json = r#"{
        "user_id": "20012",
        "nick": "Kate",
        "sex": "Female",
        "age": 24,
        "role": "Member",
        "card": "普通成员",
        "level": "L3",
        "title": null
    }"#;
    let sender: GroupSender = serde_json::from_str(json).expect("Failed to deserialize");

    assert_eq!(sender.user_id(), "20012");
    assert_eq!(sender.name(), Some("Kate"));
    assert_eq!(sender.sex(), Sex::Female);
    assert_eq!(sender.age(), 24);
    assert_eq!(sender.role(), Role::Member);
    assert_eq!(sender.card(), Some("普通成员"));
    assert_eq!(sender.level(), Some("L3"));
    assert_eq!(sender.title(), None);
}

#[test]
fn test_group_sender_round_trip_serialization() {
    let original = group_sender!(
        user_id: "20013",
        nick: "Leo",
        sex: Sex::Male,
        age: 29,
        role: Role::Owner,
        card: "群主",
        level: "MAX",
        title: "领导者"
    );
    let json = serde_json::to_string(&original).expect("Failed to serialize");
    let deserialized: GroupSender = serde_json::from_str(&json).expect("Failed to deserialize");

    assert_eq!(original.user_id(), deserialized.user_id());
    assert_eq!(original.name(), deserialized.name());
    assert_eq!(original.sex(), deserialized.sex());
    assert_eq!(original.age(), deserialized.age());
    assert_eq!(original.role(), deserialized.role());
    assert_eq!(original.card(), deserialized.card());
    assert_eq!(original.level(), deserialized.level());
    assert_eq!(original.title(), deserialized.title());
}

#[test]
fn test_group_sender_trait_object() {
    let sender = group_sender!(
        user_id: "20014",
        nick: "Mary",
        sex: Sex::Female,
        age: 23,
        role: Role::Admin
    );
    let sender_trait: &dyn Sender = &sender;

    assert_eq!(sender_trait.user_id(), "20014");
    assert_eq!(sender_trait.name(), Some("Mary"));
    assert_eq!(sender_trait.sex(), Sex::Female);
    assert_eq!(sender_trait.age(), 23);
}

#[test]
fn test_group_sender_with_all_none_optionals() {
    let sender = GroupSender {
        user_id: "20015".to_string(),
        nick: None,
        sex: Sex::Unknown,
        age: 0,
        role: Role::Unknown,
        card: None,
        level: None,
        title: None,
    };

    assert_eq!(sender.user_id(), "20015");
    assert_eq!(sender.name(), None);
    assert_eq!(sender.card(), None);
    assert_eq!(sender.level(), None);
    assert_eq!(sender.title(), None);
}

#[test]
fn test_group_sender_different_roles() {
    let owner = group_sender!(
        user_id: "20016",
        nick: "Owner1",
        sex: Sex::Male,
        age: 30,
        role: Role::Owner
    );
    assert!(owner.role().is_owner());

    let admin = group_sender!(
        user_id: "20017",
        nick: "Admin1",
        sex: Sex::Female,
        age: 25,
        role: Role::Admin
    );
    assert!(admin.role().is_admin());

    let member = group_sender!(
        user_id: "20018",
        nick: "Member1",
        sex: Sex::Unknown,
        age: 20,
        role: Role::Member
    );
    assert!(member.role().is_member());
}
