use puniyu_contact::{contact_friend, FriendContact, Scene};
#[test]
fn friend() {
	let contact = contact_friend!("123456", "test");
	assert_eq!(
		contact,
		FriendContact { scene: Scene::Friend, peer: "123456".to_string(), name: Some("test".to_string()) }
	);
}