use puniyu_adapter_core::types::SetFriendApplyType;

#[test]
fn test_set_friend_apply_type_agree() {
	let apply = SetFriendApplyType::Agree;
	let debug_str = format!("{:?}", apply);
	assert!(debug_str.contains("Agree"));
}

#[test]
fn test_set_friend_apply_type_refuse() {
	let apply = SetFriendApplyType::Refuse;
	let debug_str = format!("{:?}", apply);
	assert!(debug_str.contains("Refuse"));
}

#[test]
fn test_set_friend_apply_type_clone() {
	let apply1 = SetFriendApplyType::Agree;
	let apply2 = apply1.clone();
	assert_eq!(format!("{:?}", apply1), format!("{:?}", apply2));
}
