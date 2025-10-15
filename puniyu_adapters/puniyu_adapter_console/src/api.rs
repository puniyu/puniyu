use puniyu_core::adapter::prelude::*;

pub struct ConsoleAdapterApi;

impl AdapterApi for ConsoleAdapterApi {
	fn send_msg(&self, contact: Contact, element: Message) {
		match contact {
			Contact::Friend(friend) => {
				println!("Friend: {}", friend.name)
			}
			Contact::Group(group) => {
				println!("Group: {}", group.name)
			}
		}
	}

	fn get_avatar_url(&self, user_id: &str, size: AvatarSize) -> String {
		todo!()
	}

	fn get_group_avatar_url(&self, group_id: &str, size: AvatarSize) -> String {
		todo!()
	}
}
