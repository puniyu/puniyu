mod types;
use std::sync::Arc;
pub use types::*;

use super::{NotionBase, NotionBuilder, NotionSubEvent};
use crate::bot::Bot;
use crate::contact::{GroupContact, Scene};
use crate::event::{EventBase, EventType};
use crate::sender::GroupSender;
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
            contact: GroupContact,
            sender: GroupSender,
            content: $content_struct,
        }

        impl $struct_name {
            pub fn new(
                notion_builder: NotionBuilder<GroupContact, GroupSender>,
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
            type ContactType = GroupContact;
            type SenderType = GroupSender;

            fn bot(&self) -> &Bot { &self.bot }
            fn time(&self) -> u64 { self.time }
            fn event(&self) -> &str { EventType::Notice.into() }
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
	GroupPoke, "收到群聊戳一戳事件", NotionSubEvent::GroupPoke, GroupPokeType;
	GroupRecall, "收到群聊撤回事件", NotionSubEvent::GroupRecall, GroupRecallType;
	GroupFileUpload, "收到群聊文件上传事件", NotionSubEvent::GroupFileUpload, GroupFileUploadType;
	GroupCardChange, "收到群成员名片变动事件", NotionSubEvent::GroupCardChange, GroupCardChangeType;
	GroupMemberTitleChange, "收到群成员头衔变动事件", NotionSubEvent::GroupMemberTitleChange, GroupMemberTitleChangeType;
	GroupHighlightsChange, "收到群精华消息变动事件", NotionSubEvent::GroupHighlightsChange, GroupHighlightsChangeType;
	GroupMemberAdd, "收到群成员添加事件", NotionSubEvent::GroupMemberAdd, GroupMemberAddType;
	GroupMemberDecrease, "收到群成员减少事件", NotionSubEvent::GroupMemberDecrease, GroupMemberAddType;
	GroupAdminChange, "收到群聊管理员变动事件", NotionSubEvent::GroupAdminChange, GroupAdminChangeType;
	GroupSignIn, "收到群成员打卡事件", NotionSubEvent::GroupSignIn, ();
	GroupMemberBan, "收到群成员禁言事件", NotionSubEvent::GroupMemberBan, GroupMemberBanType;
	GroupWholeBan, "收到群全体成员禁言事件", NotionSubEvent::GroupWholeBan, GroupWholeBanType;
	GroupMessageReaction, "收到群消息表情动态事件", NotionSubEvent::GroupMessageReaction, GroupMessageReactionType;
	GroupLuckKing, "收到群运气王事件", NotionSubEvent::GroupLuckKing, GroupLuckKingType;
	GroupHonorChange, "收到群聊荣誉变更事件", NotionSubEvent::GroupHonorChange, GroupHonorChangeType;
);
