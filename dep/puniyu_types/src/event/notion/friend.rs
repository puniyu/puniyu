mod types;
pub use types::*;

use super::{NotionBase, NotionBuilder, NotionSubEvent};
use crate::contact::FriendContact;
use crate::event::{EventBase, EventType};
use crate::sender::FriendSender;

macro_rules! impl_notion_event {
    (
        $(#[$attr:meta])*
        $struct_name:ident,
        $notion_desc:expr,
        $sub_event:expr,
        $event_variant:ident,
        $content_struct:ty
    ) => {
        $(#[$attr])*
        #[derive(Debug, Clone)]
        pub struct $struct_name {
            /// 事件id
            event_id: String,
            /// 时间戳
            time: u64,
            /// BotId
            self_id: String,
            /// 用户id
            user_id: String,
            /// 事件联系人
            contact: FriendContact,
            /// 事件发送者
            sender: FriendSender,
            /// 事件内容
            content: $content_struct,
        }

        impl $struct_name {
            pub fn new(notion_builder: NotionBuilder<FriendContact, FriendSender>, content: $content_struct) -> Self {
                Self {
                    event_id: notion_builder.event_id,
                    time: notion_builder.time,
                    self_id: notion_builder.self_id,
                    user_id: notion_builder.user_id,
                    contact: notion_builder.contact,
                    sender: notion_builder.sender,
                    content,
                }
            }
        }

        impl EventBase for $struct_name {
            type ContactType = FriendContact;
            type SenderType = FriendSender;

            fn time(&self) -> u64 {
                self.time
            }

            fn event(&self) -> &str {
                EventType::Notice.into()
            }

            fn event_id(&self) -> &str {
                self.event_id.as_str()
            }

            fn sub_event(&self) -> &str {
                $sub_event.into()
            }

            fn self_id(&self) -> &str {
                self.self_id.as_str()
            }

            fn user_id(&self) -> &str {
                self.user_id.as_str()
            }

            fn contact(&self) -> Self::ContactType {
                self.contact.clone()
            }

            fn sender(&self) -> Self::SenderType {
                self.sender.clone()
            }
        }

        impl NotionBase for $struct_name {
            type Content = $content_struct;

            fn notion(&self) -> &str {
                $notion_desc
            }

            fn content(&self) -> Self::Content {
                self.content.clone()
            }
        }

        #[macro_export]
        macro_rules! $event_variant {
            (
                $adapter:expr,
                $event_id:expr,
                $time:expr,
                $self_id:expr,
                $user_id:expr,
                $contact:expr,
                $sender:expr,
                $content:expr,
            ) => {{
                let builder = NotionBuilder<FriendContact, FriendSender> {
                    event_id: $event_id.into(),
                    time: $time,
                    self_id: $self_id.into(),
                    user_id: $user_id.into(),
                    contact: $contact,
                    sender: $sender,
                };
                let notion = $struct_name::new(builder, $content);
                let event = Event::Notion(NotionEvent::$event_variant(notion));
                send_event(std::sync::Arc::from($adapter), event);
            }};
        }
    };
}

impl_notion_event!(
	ReceiveLike,
	"收到点赞事件",
	NotionSubEvent::ReceiveLike,
	create_receive_like,
	ReceiveLikeOption
);

impl_notion_event!(FriendAdd, "收到好友增加事件", NotionSubEvent::FriendAdd, create_friend_add, ());

impl_notion_event!(
	FriendDecrease,
	"收到好友减少事件",
	NotionSubEvent::FriendDecrease,
	create_friend_decrease,
	()
);

impl_notion_event!(
	PrivatePoke,
	"收到好友戳一戳事件",
	NotionSubEvent::PrivatePoke,
	create_private_poke,
	PrivatePokeOption
);

impl_notion_event!(
	PrivateRecall,
	"收到好友撤回事件",
	NotionSubEvent::PrivateRecall,
	create_private_recall,
	PrivateRecallOption
);

impl_notion_event!(
	PrivateFileUpload,
	"收到好友文件上传事件",
	NotionSubEvent::PrivateFileUpload,
	create_private_file_upload,
	PrivateFileUploadOption
);
