use puniyu_contact::{FriendContact, GroupContact, Scene};
use puniyu_element::{Elements, TextElement};
use puniyu_event::context::MessageContext;
use puniyu_event::message::{FriendMessage, GroupMessage, MessageBuilder, MessageEvent};
use puniyu_event::EventBase;
use puniyu_sender::{FriendSender, GroupSender, Role, Sex};
use std::collections::HashMap;

fn create_friend_event() -> MessageEvent {
    let builder = MessageBuilder {
        event_id: "evt_friend".to_string(),
        self_id: "bot_001".to_string(),
        user_id: "user_001".to_string(),
        contact: FriendContact {
            scene: Scene::Friend,
            peer: "user_001".to_string(),
            name: Some("Alice".to_string()),
        },
        sender: FriendSender {
            user_id: "user_001".to_string(),
            nick: Some("Alice".to_string()),
            sex: Sex::Female,
            age: 22,
        },
        time: 1234567890,
        message_id: "msg_friend".to_string(),
        elements: vec![Elements::Text(TextElement { text: "Hello".to_string() })],
    };
    MessageEvent::Friend(FriendMessage::new(builder))
}

fn create_group_event() -> MessageEvent {
    let builder = MessageBuilder {
        event_id: "evt_group".to_string(),
        self_id: "bot_001".to_string(),
        user_id: "user_002".to_string(),
        contact: GroupContact {
            scene: Scene::Group,
            peer: "group_001".to_string(),
            name: Some("TestGroup".to_string()),
        },
        sender: GroupSender {
            user_id: "user_002".to_string(),
            nick: Some("Bob".to_string()),
            sex: Sex::Male,
            age: 25,
            role: Role::Admin,
            card: None,
            level: None,
            title: None,
        },
        time: 1234567890,
        message_id: "msg_group".to_string(),
        elements: vec![Elements::Text(TextElement { text: "Hi everyone".to_string() })],
    };
    MessageEvent::Group(GroupMessage::new(builder))
}

#[test]
fn test_create() {
    let event = create_friend_event();
    let args = HashMap::new();
    let ctx = MessageContext::new(event, args);
    
    assert_eq!(ctx.event_id(), "evt_friend");
    assert_eq!(ctx.user_id(), "user_001");
}

#[test]
fn test_as_friend() {
    let event = create_friend_event();
    let ctx = MessageContext::new(event, HashMap::new());
    
    let friend = ctx.as_friend();
    assert!(friend.is_some());
    assert_eq!(friend.unwrap().user_id(), "user_001");
}

#[test]
fn test_as_friend_none() {
    let event = create_group_event();
    let ctx = MessageContext::new(event, HashMap::new());
    
    let friend = ctx.as_friend();
    assert!(friend.is_none());
}

#[test]
fn test_as_group() {
    let event = create_group_event();
    let ctx = MessageContext::new(event, HashMap::new());
    
    let group = ctx.as_group();
    assert!(group.is_some());
    assert_eq!(group.unwrap().user_id(), "user_002");
}

#[test]
fn test_as_group_none() {
    let event = create_friend_event();
    let ctx = MessageContext::new(event, HashMap::new());
    
    let group = ctx.as_group();
    assert!(group.is_none());
}

#[test]
fn test_arg() {
    let event = create_friend_event();
    let mut args = HashMap::new();
    args.insert("name".to_string(), Some("Alice".to_string()));
    args.insert("age".to_string(), Some("22".to_string()));
    args.insert("empty".to_string(), None);
    
    let ctx = MessageContext::new(event, args);
    
    assert_eq!(ctx.arg("name"), Some("Alice".to_string()));
    assert_eq!(ctx.arg("age"), Some("22".to_string()));
    assert_eq!(ctx.arg("empty"), None);
    assert_eq!(ctx.arg("not_exists"), None);
}

#[test]
fn test_arg_empty() {
    let event = create_friend_event();
    let ctx = MessageContext::new(event, HashMap::new());
    
    assert_eq!(ctx.arg("anything"), None);
}

#[test]
fn test_event_methods() {
    let event = create_friend_event();
    let ctx = MessageContext::new(event, HashMap::new());
    
    assert_eq!(ctx.event(), "message");
    assert_eq!(ctx.sub_event(), "friend");
    assert_eq!(ctx.self_id(), "bot_001");
}

#[test]
fn test_clone() {
    let event = create_friend_event();
    let mut args = HashMap::new();
    args.insert("key".to_string(), Some("value".to_string()));
    
    let ctx = MessageContext::new(event, args);
    let cloned = ctx.clone();
    
    assert_eq!(ctx.event_id(), cloned.event_id());
    assert_eq!(ctx.arg("key"), cloned.arg("key"));
}

#[test]
fn test_contact() {
    let event = create_friend_event();
    let ctx = MessageContext::new(event, HashMap::new());
    
    let contact = ctx.contact();
    match contact {
        puniyu_contact::ContactType::Friend(f) => {
            assert_eq!(f.peer, "user_001");
        }
        _ => panic!("Expected Friend contact"),
    }
}

#[test]
fn test_sender() {
    let event = create_friend_event();
    let ctx = MessageContext::new(event, HashMap::new());
    
    let sender = ctx.sender();
    match sender {
        puniyu_sender::SenderType::Friend(s) => {
            assert_eq!(s.user_id, "user_001");
        }
        _ => panic!("Expected Friend sender"),
    }
}

#[test]
fn test_multiple_args() {
    let event = create_friend_event();
    let mut args = HashMap::new();
    args.insert("cmd".to_string(), Some("test".to_string()));
    args.insert("param1".to_string(), Some("value1".to_string()));
    args.insert("param2".to_string(), Some("value2".to_string()));
    
    let ctx = MessageContext::new(event, args);
    
    assert_eq!(ctx.arg("cmd"), Some("test".to_string()));
    assert_eq!(ctx.arg("param1"), Some("value1".to_string()));
    assert_eq!(ctx.arg("param2"), Some("value2".to_string()));
}
