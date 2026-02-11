use puniyu_adapter_core::types::{MessageType, SendMsgType};

#[test]
fn test_message_type_from_string() {
	let msg_type = MessageType::from("msg_123456".to_string());
	match msg_type {
		MessageType::Id(id) => assert_eq!(id, "msg_123456"),
		_ => panic!("Expected MessageType::Id"),
	}
}

#[test]
fn test_message_type_from_str() {
	let msg_type = MessageType::from("msg_123456");
	match msg_type {
		MessageType::Id(id) => assert_eq!(id, "msg_123456"),
		_ => panic!("Expected MessageType::Id"),
	}
}

#[test]
fn test_message_type_from_u64() {
	let msg_type = MessageType::from(123456u64);
	match msg_type {
		MessageType::Seq(seq) => assert_eq!(seq, 123456),
		_ => panic!("Expected MessageType::Seq"),
	}
}

#[test]
fn test_message_type_id_variant() {
	let msg_type = MessageType::Id("test_id".to_string());
	match msg_type {
		MessageType::Id(id) => assert_eq!(id, "test_id"),
		_ => panic!("Expected MessageType::Id"),
	}
}

#[test]
fn test_message_type_seq_variant() {
	let msg_type = MessageType::Seq(999);
	match msg_type {
		MessageType::Seq(seq) => assert_eq!(seq, 999),
		_ => panic!("Expected MessageType::Seq"),
	}
}

#[test]
fn test_message_type_clone() {
	let msg_type1 = MessageType::Id("test".to_string());
	let msg_type2 = msg_type1.clone();

	match (msg_type1, msg_type2) {
		(MessageType::Id(id1), MessageType::Id(id2)) => assert_eq!(id1, id2),
		_ => panic!("Expected both to be MessageType::Id"),
	}
}

#[test]
fn test_message_type_debug() {
	let msg_type = MessageType::Id("test".to_string());
	let debug_str = format!("{:?}", msg_type);
	assert!(debug_str.contains("Id"));
	assert!(debug_str.contains("test"));
}

#[test]
fn test_send_msg_type_creation() {
	let send_msg = SendMsgType { message_id: "msg_123".to_string(), time: 1234567890 };

	assert_eq!(send_msg.message_id, "msg_123");
	assert_eq!(send_msg.time, 1234567890);
}

#[test]
fn test_send_msg_type_clone() {
	let send_msg1 = SendMsgType { message_id: "msg_123".to_string(), time: 1234567890 };
	let send_msg2 = send_msg1.clone();

	assert_eq!(send_msg1.message_id, send_msg2.message_id);
	assert_eq!(send_msg1.time, send_msg2.time);
}

#[test]
fn test_send_msg_type_debug() {
	let send_msg = SendMsgType { message_id: "msg_123".to_string(), time: 1234567890 };
	let debug_str = format!("{:?}", send_msg);

	assert!(debug_str.contains("SendMsgType"));
	assert!(debug_str.contains("msg_123"));
	assert!(debug_str.contains("1234567890"));
}

#[test]
fn test_message_type_empty_string() {
	let msg_type = MessageType::from("");
	match msg_type {
		MessageType::Id(id) => assert_eq!(id, ""),
		_ => panic!("Expected MessageType::Id"),
	}
}

#[test]
fn test_message_type_zero_seq() {
	let msg_type = MessageType::from(0u64);
	match msg_type {
		MessageType::Seq(seq) => assert_eq!(seq, 0),
		_ => panic!("Expected MessageType::Seq"),
	}
}

#[test]
fn test_message_type_large_seq() {
	let msg_type = MessageType::from(u64::MAX);
	match msg_type {
		MessageType::Seq(seq) => assert_eq!(seq, u64::MAX),
		_ => panic!("Expected MessageType::Seq"),
	}
}

#[test]
fn test_send_msg_type_zero_time() {
	let send_msg = SendMsgType { message_id: "msg_123".to_string(), time: 0 };
	assert_eq!(send_msg.time, 0);
}

#[test]
fn test_send_msg_type_empty_id() {
	let send_msg = SendMsgType { message_id: "".to_string(), time: 1234567890 };
	assert_eq!(send_msg.message_id, "");
}

#[test]
fn test_message_type_unicode_id() {
	let msg_type = MessageType::from("消息_123");
	match msg_type {
		MessageType::Id(id) => assert_eq!(id, "消息_123"),
		_ => panic!("Expected MessageType::Id"),
	}
}

#[test]
fn test_message_type_special_chars() {
	let msg_type = MessageType::from("msg@#$%^&*()");
	match msg_type {
		MessageType::Id(id) => assert_eq!(id, "msg@#$%^&*()"),
		_ => panic!("Expected MessageType::Id"),
	}
}
