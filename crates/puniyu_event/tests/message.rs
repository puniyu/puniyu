use puniyu_contact::{FriendContact, GroupContact, Scene};
use puniyu_element::{AtElement, Elements, ImageElement, RecordElement, ReplyElement, TextElement};
use puniyu_event::message::{FriendMessage, GroupMessage, MessageBase, MessageBuilder};
use puniyu_event::EventBase;
use puniyu_sender::{FriendSender, GroupSender, Role, Sex};

fn text(s: &str) -> Elements {
    Elements::Text(TextElement { text: s.to_string() })
}

fn create_friend_message(elements: Vec<Elements>) -> FriendMessage {
    let builder = MessageBuilder {
        event_id: "evt_001".to_string(),
        self_id: "bot_123".to_string(),
        user_id: "user_456".to_string(),
        contact: FriendContact {
            scene: Scene::Friend,
            peer: "user_456".to_string(),
            name: Some("TestUser".to_string()),
        },
        sender: FriendSender {
            user_id: "user_456".to_string(),
            nick: Some("TestUser".to_string()),
            sex: Sex::Unknown,
            age: 20,
        },
        time: 1234567890,
        message_id: "msg_001".to_string(),
        elements,
    };
    FriendMessage::new(builder)
}

fn create_group_message(elements: Vec<Elements>) -> GroupMessage {
    let builder = MessageBuilder {
        event_id: "evt_002".to_string(),
        self_id: "bot_123".to_string(),
        user_id: "user_789".to_string(),
        contact: GroupContact {
            scene: Scene::Group,
            peer: "group_999".to_string(),
            name: Some("TestGroup".to_string()),
        },
        sender: GroupSender {
            user_id: "user_789".to_string(),
            nick: Some("GroupUser".to_string()),
            sex: Sex::Male,
            age: 25,
            role: Role::Member,
            card: Some("Member".to_string()),
            level: Some("L5".to_string()),
            title: None,
        },
        time: 1234567890,
        message_id: "msg_002".to_string(),
        elements,
    };
    GroupMessage::new(builder)
}

#[test]
fn test_friend_create() {
    let msg = create_friend_message(vec![text("Hello")]);
    assert_eq!(msg.event_id(), "evt_001");
    assert_eq!(msg.self_id(), "bot_123");
    assert_eq!(msg.user_id(), "user_456");
    assert_eq!(msg.message_id(), "msg_001");
}

#[test]
fn test_friend_event() {
    let msg = create_friend_message(vec![]);
    assert_eq!(msg.event(), "message");
    assert_eq!(msg.sub_event(), "friend");
    assert_eq!(msg.time(), 1234567890);
}

#[test]
fn test_friend_contact() {
    let msg = create_friend_message(vec![]);
    let contact = msg.contact();
    assert_eq!(contact.peer, "user_456");
    assert_eq!(contact.name, Some("TestUser".to_string()));
}

#[test]
fn test_friend_sender() {
    let msg = create_friend_message(vec![]);
    let sender = msg.sender();
    assert_eq!(sender.user_id, "user_456");
    assert_eq!(sender.nick, Some("TestUser".to_string()));
}

#[test]
fn test_friend_elements() {
    let elements = vec![text("Hello"), text("World")];
    let msg = create_friend_message(elements.clone());
    let result = msg.elements();
    assert_eq!(result.len(), 2);
}

#[test]
fn test_group_create() {
    let msg = create_group_message(vec![text("Hi")]);
    assert_eq!(msg.event_id(), "evt_002");
    assert_eq!(msg.self_id(), "bot_123");
    assert_eq!(msg.user_id(), "user_789");
    assert_eq!(msg.message_id(), "msg_002");
}

#[test]
fn test_group_event() {
    let msg = create_group_message(vec![]);
    assert_eq!(msg.event(), "message");
    assert_eq!(msg.sub_event(), "group");
}

#[test]
fn test_group_contact() {
    let msg = create_group_message(vec![]);
    let contact = msg.contact();
    assert_eq!(contact.peer, "group_999");
}

#[test]
fn test_get_at() {
    let elements = vec![
        Elements::At(AtElement {
            target_id: "user_123".to_string(),
            name: None,
        }),
        text("Hello"),
        Elements::At(AtElement {
            target_id: "user_456".to_string(),
            name: None,
        }),
    ];
    let msg = create_friend_message(elements);
    let at_list = msg.get_at();
    assert_eq!(at_list.len(), 2);
    assert!(at_list.contains(&"user_123".to_string()));
    assert!(at_list.contains(&"user_456".to_string()));
}

#[test]
fn test_get_at_empty() {
    let msg = create_friend_message(vec![text("No at here")]);
    let at_list = msg.get_at();
    assert_eq!(at_list.len(), 0);
}

#[test]
fn test_get_at_all() {
    let elements = vec![Elements::At(AtElement {
        target_id: "all".to_string(),
        name: None,
    })];
    let msg = create_friend_message(elements);
    assert!(msg.get_at_all());
}

#[test]
fn test_get_at_all_false() {
    let elements = vec![Elements::At(AtElement {
        target_id: "user_123".to_string(),
        name: None,
    })];
    let msg = create_friend_message(elements);
    assert!(!msg.get_at_all());
}

#[test]
fn test_get_at_bot() {
    let elements = vec![Elements::At(AtElement {
        target_id: "bot_123".to_string(),
        name: None,
    })];
    let msg = create_friend_message(elements);
    assert!(msg.get_at_bot());
}

#[test]
fn test_get_at_bot_false() {
    let elements = vec![Elements::At(AtElement {
        target_id: "user_789".to_string(),
        name: None,
    })];
    let msg = create_friend_message(elements);
    assert!(!msg.get_at_bot());
}

#[test]
fn test_get_image() {
    let image_data = vec![1, 2, 3, 4, 5];
    let elements = vec![
        text("Check this"),
        Elements::Image(ImageElement {
            file: image_data.clone(),
            is_flash: false,
            summary: None,
        }),
    ];
    let msg = create_friend_message(elements);
    let result = msg.get_image();
    assert!(result.is_some());
    assert_eq!(result.unwrap(), image_data);
}

#[test]
fn test_get_image_none() {
    let msg = create_friend_message(vec![text("No image")]);
    assert!(msg.get_image().is_none());
}

#[test]
fn test_get_record() {
    let audio_data = vec![10, 20, 30];
    let elements = vec![Elements::Record(RecordElement {
        file: audio_data.clone(),
    })];
    let msg = create_friend_message(elements);
    let result = msg.get_record();
    assert!(result.is_some());
    assert_eq!(result.unwrap(), audio_data);
}

#[test]
fn test_get_record_none() {
    let msg = create_friend_message(vec![text("No record")]);
    assert!(msg.get_record().is_none());
}

#[test]
fn test_get_reply_id() {
    let elements = vec![
        Elements::Reply(ReplyElement {
            message_id: "reply_to_123".to_string(),
        }),
        text("Reply content"),
    ];
    let msg = create_friend_message(elements);
    let reply_id = msg.get_reply_id();
    assert!(reply_id.is_some());
    assert_eq!(reply_id.unwrap(), "reply_to_123");
}

#[test]
fn test_get_reply_id_none() {
    let msg = create_friend_message(vec![text("Not a reply")]);
    assert!(msg.get_reply_id().is_none());
}

#[test]
fn test_mixed_elements() {
    let elements = vec![
        text("Hello"),
        Elements::At(AtElement {
            target_id: "user_001".to_string(),
            name: None,
        }),
        Elements::Image(ImageElement {
            file: vec![1, 2, 3],
            is_flash: false,
            summary: None,
        }),
        Elements::Reply(ReplyElement {
            message_id: "msg_999".to_string(),
        }),
    ];
    let msg = create_friend_message(elements);
    
    assert_eq!(msg.get_at().len(), 1);
    assert!(msg.get_image().is_some());
    assert!(msg.get_reply_id().is_some());
}

#[test]
fn test_clone() {
    let msg = create_friend_message(vec![text("Test")]);
    let cloned = msg.clone();
    
    assert_eq!(msg.event_id(), cloned.event_id());
    assert_eq!(msg.message_id(), cloned.message_id());
    assert_eq!(msg.user_id(), cloned.user_id());
}
