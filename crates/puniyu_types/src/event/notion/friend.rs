mod types;

use std::sync::Arc;
pub use types::*;

use super::{NotionBase, NotionBuilder, NotionSubEvent};
use crate::bot::Bot;
use crate::contact::{FriendContact, Scene};
use crate::event::{EventBase, EventType};
use crate::sender::FriendSender;
use serde::{Deserialize, Serialize};
use crate::event::inner::{deserialize_bot, serialize_bot};

macro_rules! impl_notion_event {
    (
        $(#[$attr:meta])*
        $struct_name:ident,
        $notion_desc:expr,
        $sub_event:expr,
        $content_struct:ty
        $(;)?
    ) => {
        impl_notion_event!(
            @impl
            $(#[$attr])*
            $struct_name,
            $notion_desc,
            $sub_event,
            $content_struct
        );
    };

    (
        $(
            $(#[$attr:meta])*
            $struct_name:ident,
            $notion_desc:expr,
            $sub_event:expr,
            $content_struct:ty;
        )+
    ) => {
        $(
            impl_notion_event!(
                @impl
                $(#[$attr])*
                $struct_name,
                $notion_desc,
                $sub_event,
                $content_struct
            );
        )+
    };

    (@impl
        $(#[$attr:meta])*
        $struct_name:ident,
        $notion_desc:expr,
        $sub_event:expr,
        $content_struct:ty
    ) => {
        $(#[$attr])*
        #[derive(Debug, Clone, Deserialize, Serialize)]
        pub struct $struct_name {
            #[serde(
				serialize_with = "serialize_bot",
				deserialize_with = "deserialize_bot"
			)]
            bot: Arc<Bot>,
            event_id: String,
            time: u64,
            self_id: String,
            user_id: String,
            contact: FriendContact,
            sender: FriendSender,
            content: $content_struct,
        }

        impl $struct_name {
            pub fn new(
                notion_builder: NotionBuilder<FriendContact, FriendSender>,
                content: $content_struct
            ) -> Self {
                Self {
                    bot: notion_builder.bot,
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
            fn bot(&self) -> &Bot { &self.bot }
            fn time(&self) -> u64 { self.time }
            fn event(&self) -> &str { EventType::Notion.into() }
            fn event_id(&self) -> &str { &self.event_id }
            fn sub_event(&self) -> &str { $sub_event.into() }
            fn self_id(&self) -> &str { &self.self_id }
            fn user_id(&self) -> &str { &self.user_id }
            fn contact(&self) -> Self::ContactType { self.contact.clone() }
            fn sender(&self) -> Self::SenderType { self.sender.clone() }
            fn is_friend(&self) -> bool { matches!(self.contact().scene, Scene::Friend) }
            fn is_group(&self) -> bool { matches!(self.contact().scene, Scene::Group) }
        }

        impl NotionBase for $struct_name {
            type Content = $content_struct;
            fn notion(&self) -> &str { $notion_desc }
            fn content(&self) -> Self::Content { self.content.clone() }
        }
    };
}

impl_notion_event!(
	ReceiveLike, "收到点赞事件", NotionSubEvent::ReceiveLike, ReceiveLikeOption;
	FriendAdd, "收到好友增加事件", NotionSubEvent::FriendAdd, ();
	FriendDecrease, "收到好友减少事件", NotionSubEvent::FriendDecrease, ();
	PrivatePoke, "收到好友戳一戳事件", NotionSubEvent::PrivatePoke, PrivatePokeOption;
	PrivateRecall, "收到好友撤回事件", NotionSubEvent::PrivateRecall, PrivateRecallOption;
	PrivateFileUpload, "收到好友文件上传事件", NotionSubEvent::PrivateFileUpload, PrivateFileUploadOption;
);
