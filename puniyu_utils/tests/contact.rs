use puniyu_utils::contact::Scene;
#[test]
fn friend() {
	use puniyu_utils::{contact::FriendContact, contact_friend};
	let contact = contact_friend!(
		peer: "114514",
		name: "猪咪"
	);

	let FriendContact { scene, peer, name } = contact;
	assert_eq!(scene, Scene::Friend);
	assert!(!peer.is_empty());
	assert!(!name.is_empty());
}

#[test]
fn group() {
	use puniyu_utils::{contact::GroupContact, contact_group};
	let contact = contact_group!(
		peer: "114514",
		name: "猪咪"
	);

	let GroupContact { scene, peer, name } = contact;
	assert_eq!(scene, Scene::Group);
	assert!(!peer.is_empty());
	assert!(!name.is_empty());
}
