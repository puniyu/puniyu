use puniyu_contact::{GroupContact, Scene, contact_group};

#[test]
fn group() {
	let contact = contact_group!("123456");
	assert_eq!(
		contact,
		GroupContact { scene: Scene::Group, peer: "123456".to_string(), name: None }
	);
}
