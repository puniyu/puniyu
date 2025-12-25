mod types;
pub use types::*;

use super::{NotionBase, NotionBuilder, NotionSubEvent};
use crate::contact::GroupContact;
use crate::event::{EventBase, EventType};
use crate::sender::GroupSender;
use serde::{Deserialize, Serialize};

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
        #[derive(Debug, Clone,Deserialize, Serialize)]
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
            contact: GroupContact,
            /// 事件发送者
            sender: GroupSender,
            /// 事件内容
            content: $content_struct,
        }

        impl $struct_name {
            pub fn new(notion_builder: NotionBuilder<GroupContact, GroupSender>, content: $content_struct) -> Self {
                Self {
                    event_id: notion_builder.event_id,
                    time: notion_builder.time,
                    self_id: notion_builder.self_id,
                    user_id: notion_builder.user_id,
                    contact: notion_builder.contact,
                    sender: notion_builder.sender,
                    content
                }
            }

	        pub fn group_id(&self) -> &str {
                &self.contact.peer
            }
        }

        impl EventBase for $struct_name {
            type ContactType = GroupContact;
            type SenderType = GroupSender;

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

        #[cfg(feature = "event")]
        #[macro_export]
        macro_rules! $event_variant {
            (
                $adapter:expr,
                $event_id:expr,
                $time: expr,
                $self_id:expr,
                $user_id:expr,
                $group_id:expr,
                $contact:expr,
                $sender:expr,
                $content:expr,
            ) => {{
                let builder = NotionBuilder<GroupContact, GroupSender> {
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
	GroupPoke,
	"收到群聊戳一戳事件",
	NotionSubEvent::GroupPoke,
	create_group_poke,
	GroupPokeType
);


impl_notion_event!(
	GroupRecall,
	"收到群聊撤回事件",
	NotionSubEvent::GroupRecall,
	create_group_recall,
	GroupRecallType
);


impl_notion_event!(
	GroupFileUpload,
	"收到群聊文件上传事件",
	NotionSubEvent::GroupFileUpload,
	create_group_file_upload,
	GroupFileUploadType
);



impl_notion_event!(
	GroupCardChange,
	"收到群成员名片变动事件",
	NotionSubEvent::GroupCardChange,
	create_group_card_change,
	GroupCardChangeType
);


impl_notion_event!(
	GroupMemberTitleChange,
	"收到群成员头衔变动事件",
	NotionSubEvent::GroupMemberTitleChange,
	create_group_member_title_change,
	GroupMemberTitleChangeType
);


impl_notion_event!(
	GroupHighlightsChange,
	"收到群精华消息变动事件",
	NotionSubEvent::GroupHighlightsChange,
	create_group_highlights_change,
	GroupHighlightsChangeType
);



impl_notion_event!(
	GroupMemberAdd,
	"收到群成员添加事件",
	NotionSubEvent::GroupMemberAdd,
	create_group_memeber_add,
	GroupMemberAddType
);


impl_notion_event!(
	GroupMemberDecrease,
	"收到群成员减少事件",
	NotionSubEvent::GroupMemberDecrease,
	create_group_memeber_decrease,
	GroupMemberAddType
);


impl_notion_event!(
	GroupAdminChange,
	"收到群聊管理员变动事件",
	NotionSubEvent::GroupAdminChange,
	create_group_admin_change,
	GroupAdminChangeType
);

impl_notion_event!(
	GroupSignIn,
	"收到群成员打卡事件",
	NotionSubEvent::GroupSignIn,
	create_group_sign_in,
	()
);


impl_notion_event!(
	GroupMemberBan,
	"收到群成员禁言事件",
	NotionSubEvent::GroupMemberBan,
	create_group_member_ban,
	GroupMemberBanType
);



impl_notion_event!(
	GroupWholeBan,
	"收到群全体成员禁言事件",
	NotionSubEvent::GroupWholeBan,
	create_group_whole_ban,
	GroupWholeBanType
);


impl_notion_event!(
	GroupMessageReaction,
	"收到群消息表情动态事件",
	NotionSubEvent::GroupMessageReaction,
	create_group_message_reaction,
	GroupMessageReactionType
);



impl_notion_event!(
	GroupLuckKing,
	"收到群运气王事件",
	NotionSubEvent::GroupLuckKing,
	create_group_luck_king,
	GroupLuckKingType
);

impl_notion_event!(
	GroupHonorChange,
	"收到群聊荣誉变更事件",
	NotionSubEvent::GroupHonorChange,
	create_group_honor_change,
	GroupHonorChangeType
);
