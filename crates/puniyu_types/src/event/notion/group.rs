mod types;
pub use types::*;

use super::{NotionBase, NotionBuilder, NotionSubEvent};
use crate::bot::BotInfo;
use crate::contact::{GroupContact, Scene};
use crate::event::{EventBase, EventType};
use crate::sender::GroupSender;
use serde::{Serialize, Deserialize};

macro_rules! impl_notion_event {
    (
        $(#[$attr:meta])*
        $struct_name:ident,
        $notion_desc:expr,
        $sub_event:expr,
        $event_variant:ident,
        $content_struct:ty
        $(;)?
    ) => {
        impl_notion_event!(
            @impl
            $(#[$attr])*
            $struct_name,
            $notion_desc,
            $sub_event,
            $event_variant,
            $content_struct
        );
    };

    (
        $(
            $(#[$attr:meta])*
            $struct_name:ident,
            $notion_desc:expr,
            $sub_event:expr,
            $event_variant:ident,
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
                $event_variant,
                $content_struct
            );
        )+
    };

    (@impl
        $(#[$attr:meta])*
        $struct_name:ident,
        $notion_desc:expr,
        $sub_event:expr,
        $event_variant:ident,
        $content_struct:ty
    ) => {
        $(#[$attr])*
        #[derive(Debug, Clone, Deserialize, Serialize)]
        pub struct $struct_name {
            bot: BotInfo,
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
            
            fn bot(&self) -> &BotInfo { &self.bot }
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


#[cfg(feature = "event")]
#[macro_export]
macro_rules! create_notion_event {
    (
        $variant:ident,
        $( $key:ident : $value:expr ),* $(,)?
    ) => {{
        let mut builder = NotionBuilder {
            event_id: String::new(),
            time: 0,
            self_id: String::new(),
            user_id: String::new(),
            contact: Default::default(),
            sender: Default::default(),
        };

        $(
            builder.$key = create_notion_event!(@convert $key, $value);
        )*

        let notion = $variant::new(builder, builder.content.clone());
        let event = Event::Notion(NotionEvent::$variant(notion));

        send_event(std::sync::Arc::clone(&builder.adapter), event);
    }};

    (@convert adapter, $v:expr) => { $v };
    (@convert event_id, $v:expr) => { $v.to_string() };
    (@convert time, $v:expr) => { $v };
    (@convert self_id, $v:expr) => { $v.to_string() };
    (@convert user_id, $v:expr) => { $v.to_string() };
    (@convert contact, $v:expr) => { $v };
    (@convert sender, $v:expr) => { $v };
    (@convert content, $v:expr) => { $v };
}


impl_notion_event!(
	GroupPoke, "收到群聊戳一戳事件", NotionSubEvent::GroupPoke, create_group_poke, GroupPokeType;
	GroupRecall, "收到群聊撤回事件", NotionSubEvent::GroupRecall, create_group_recall, GroupRecallType;
	GroupFileUpload, "收到群聊文件上传事件", NotionSubEvent::GroupFileUpload, create_group_file_upload, GroupFileUploadType;
	GroupCardChange, "收到群成员名片变动事件", NotionSubEvent::GroupCardChange, create_group_card_change, GroupCardChangeType;
	GroupMemberTitleChange, "收到群成员头衔变动事件", NotionSubEvent::GroupMemberTitleChange, create_group_member_title_change, GroupMemberTitleChangeType;
	GroupHighlightsChange, "收到群精华消息变动事件", NotionSubEvent::GroupHighlightsChange, create_group_highlights_change, GroupHighlightsChangeType;
	GroupMemberAdd, "收到群成员添加事件", NotionSubEvent::GroupMemberAdd, create_group_memeber_add, GroupMemberAddType;
	GroupMemberDecrease, "收到群成员减少事件", NotionSubEvent::GroupMemberDecrease, create_group_memeber_decrease, GroupMemberAddType;
	GroupAdminChange, "收到群聊管理员变动事件", NotionSubEvent::GroupAdminChange, create_group_admin_change, GroupAdminChangeType;
	GroupSignIn, "收到群成员打卡事件", NotionSubEvent::GroupSignIn, create_group_sign_in, ();
	GroupMemberBan, "收到群成员禁言事件", NotionSubEvent::GroupMemberBan, create_group_member_ban, GroupMemberBanType;
	GroupWholeBan, "收到群全体成员禁言事件", NotionSubEvent::GroupWholeBan, create_group_whole_ban, GroupWholeBanType;
	GroupMessageReaction, "收到群消息表情动态事件", NotionSubEvent::GroupMessageReaction, create_group_message_reaction, GroupMessageReactionType;
	GroupLuckKing, "收到群运气王事件", NotionSubEvent::GroupLuckKing, create_group_luck_king, GroupLuckKingType;
	GroupHonorChange, "收到群聊荣誉变更事件", NotionSubEvent::GroupHonorChange, create_group_honor_change, GroupHonorChangeType;
);
