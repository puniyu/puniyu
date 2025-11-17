use puniyu_sender::{friend_sender, FriendSender, Sender, Sex};

#[test]
fn test_friend_sender_full_params() {
    let sender = friend_sender!(
        user_id: "10001",
        nick: "Alice",
        sex: Sex::Female,
        age: 20
    );

    assert_eq!(sender.user_id(), "10001");
    assert_eq!(sender.name(), Some("Alice"));
    assert_eq!(sender.sex(), Sex::Female);
    assert_eq!(sender.age(), 20);
}

#[test]
fn test_friend_sender_positional_params() {
    let sender = friend_sender!("10002", "Bob", Sex::Male, 25);

    assert_eq!(sender.user_id(), "10002");
    assert_eq!(sender.name(), Some("Bob"));
    assert_eq!(sender.sex(), Sex::Male);
    assert_eq!(sender.age(), 25);
}

#[test]
fn test_friend_sender_without_age() {
    let sender = friend_sender!("10003", "Charlie", Sex::Unknown);

    assert_eq!(sender.user_id(), "10003");
    assert_eq!(sender.name(), Some("Charlie"));
    assert_eq!(sender.sex(), Sex::Unknown);
    assert_eq!(sender.age(), 0);
}

#[test]
fn test_friend_sender_without_sex_and_age() {
    let sender = friend_sender!("10004", "David");

    assert_eq!(sender.user_id(), "10004");
    assert_eq!(sender.name(), Some("David"));
    assert!(sender.sex().is_unknown());
    assert_eq!(sender.age(), 0);
}

#[test]
fn test_friend_sender_only_user_id() {
    let sender = friend_sender!("10005");

    assert_eq!(sender.user_id(), "10005");
    assert_eq!(sender.name(), None);
    assert!(sender.sex().is_unknown());
    assert_eq!(sender.age(), 0);
}

#[test]
fn test_friend_sender_direct_construction() {
    let sender = FriendSender {
        user_id: "10006".to_string(),
        nick: Some("Eve".to_string()),
        sex: Sex::Female,
        age: 18,
    };

    assert_eq!(sender.user_id(), "10006");
    assert_eq!(sender.name(), Some("Eve"));
    assert_eq!(sender.sex(), Sex::Female);
    assert_eq!(sender.age(), 18);
}

#[test]
fn test_friend_sender_clone() {
    let sender = friend_sender!("10007", "Frank", Sex::Male, 30);
    let cloned = sender.clone();

    assert_eq!(sender.user_id(), cloned.user_id());
    assert_eq!(sender.name(), cloned.name());
    assert_eq!(sender.sex(), cloned.sex());
    assert_eq!(sender.age(), cloned.age());
}

#[test]
fn test_sex_is_methods() {
    assert!(Sex::Male.is_male());
    assert!(!Sex::Male.is_female());
    assert!(!Sex::Male.is_unknown());

    assert!(Sex::Female.is_female());
    assert!(!Sex::Female.is_male());
    assert!(!Sex::Female.is_unknown());

    assert!(Sex::Unknown.is_unknown());
    assert!(!Sex::Unknown.is_male());
    assert!(!Sex::Unknown.is_female());
}

#[test]
fn test_friend_sender_serialization() {
    let sender = friend_sender!("10008", "Grace", Sex::Female, 22);
    let json = serde_json::to_string(&sender).expect("Failed to serialize");

    assert!(json.contains("10008"));
    assert!(json.contains("Grace"));
}

#[test]
fn test_friend_sender_deserialization() {
    let json = r#"{"user_id":"10009","nick":"Henry","sex":"Male","age":28}"#;
    let sender: FriendSender = serde_json::from_str(json).expect("Failed to deserialize");

    assert_eq!(sender.user_id(), "10009");
    assert_eq!(sender.name(), Some("Henry"));
    assert_eq!(sender.sex(), Sex::Male);
    assert_eq!(sender.age(), 28);
}

#[test]
fn test_friend_sender_round_trip_serialization() {
    let original = friend_sender!("10010", "Ivy", Sex::Female, 19);
    let json = serde_json::to_string(&original).expect("Failed to serialize");
    let deserialized: FriendSender = serde_json::from_str(&json).expect("Failed to deserialize");

    assert_eq!(original.user_id(), deserialized.user_id());
    assert_eq!(original.name(), deserialized.name());
    assert_eq!(original.sex(), deserialized.sex());
    assert_eq!(original.age(), deserialized.age());
}

#[test]
fn test_friend_sender_with_none_nick() {
    let sender = FriendSender {
        user_id: "10011".to_string(),
        nick: None,
        sex: Sex::Unknown,
        age: 0,
    };

    assert_eq!(sender.user_id(), "10011");
    assert_eq!(sender.name(), None);
}

#[test]
fn test_friend_sender_trait_object() {
    let sender = friend_sender!("10012", "Jack", Sex::Male, 35);
    let sender_trait: &dyn Sender = &sender;

    assert_eq!(sender_trait.user_id(), "10012");
    assert_eq!(sender_trait.name(), Some("Jack"));
    assert_eq!(sender_trait.sex(), Sex::Male);
    assert_eq!(sender_trait.age(), 35);
}
