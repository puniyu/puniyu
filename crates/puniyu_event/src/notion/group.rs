use crate::notion::{NotionBase, NotionBuilder, NotionSubEvent};
use crate::{EventBase, EventType};
use puniyu_adapter_api::types::HighlightsAction;
use puniyu_contact::GroupContact;
use puniyu_sender::GroupSender;

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
            contact: GroupContact,
            /// 事件发送者
            sender: GroupSender,
            /// 事件内容
            content: $content_struct,
        }

        impl $struct_name {
            pub fn new(notion_builder: NotionBuilder<GroupContact, GroupSender>, content: $content_struct) -> Self {
                use std::time::{SystemTime, UNIX_EPOCH};
                let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                Self {
                    event_id: notion_builder.event_id,
                    time: timestamp,
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

        #[cfg(feature = "notion")]
        #[macro_export]
        macro_rules! $event_variant {
            (
                $adapter:expr,
                $event_id:expr,
                $self_id:expr,
                $user_id:expr,
                $group_id:expr,
                $contact:expr,
                $sender:expr,
                $content:expr,
            ) => {{
                let builder = NotionBuilder<GroupContact, GroupSender> {
                    event_id: $event_id.into(),
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

#[derive(Debug, Clone)]
pub struct GroupPokeType {
	/// 被戳的用户id
	pub target_id: String,
}

impl_notion_event!(
	GroupPoke,
	"收到群聊戳一戳事件",
	NotionSubEvent::GroupPoke,
	create_group_poke,
	GroupPokeType
);

#[derive(Debug, Clone)]
pub struct GroupRecallType {
	pub message_id: String,
}

impl_notion_event!(
	GroupRecall,
	"收到群聊撤回事件",
	NotionSubEvent::GroupRecall,
	create_group_recall,
	GroupRecallType
);

#[derive(Debug, Clone)]
pub struct GroupFileUploadType {
	/// 文件id
	pub file_id: String,
	/// 文件名
	pub file_name: String,
	/// 文件大小
	pub file_size: u64,
}

impl_notion_event!(
	GroupFileUpload,
	"收到群聊文件上传事件",
	NotionSubEvent::GroupFileUpload,
	create_group_file_upload,
	GroupFileUploadType
);

#[derive(Debug, Clone)]
pub struct GroupCardChangeType {
	/// 群名片，新的群名片
	pub card: String,
}

impl_notion_event!(
	GroupCardChange,
	"收到群成员名片变动事件",
	NotionSubEvent::GroupCardChange,
	create_group_card_change,
	GroupCardChangeType
);

#[derive(Debug, Clone)]
pub struct GroupMemberTitleChangeType {
	pub title: String,
}

impl_notion_event!(
	GroupMemberTitleChange,
	"收到群成员头衔变动事件",
	NotionSubEvent::GroupMemberTitleChange,
	create_group_member_title_change,
	GroupMemberTitleChangeType
);

#[derive(Debug, Clone)]
pub struct GroupHighlightsChangeType {
	/// 被添加精华消息的用户id
	pub target_id: String,
	pub message_id: String,
	pub action: HighlightsAction,
}

impl_notion_event!(
	GroupHighlightsChange,
	"收到群精华消息变动事件",
	NotionSubEvent::GroupHighlightsChange,
	create_group_highlights_change,
	GroupHighlightsChangeType
);

#[derive(Debug, Clone)]
pub struct GroupMemberAddType {
	/// 加入的用户id
	pub target_id: String,
	pub join_type: GroupJoinType,
}

#[derive(Debug, Clone)]
pub enum GroupJoinType {
	/// 邀请
	Invite,
	/// 群管理同意
	Apply,
}

impl_notion_event!(
	GroupMemberAdd,
	"收到群成员添加事件",
	NotionSubEvent::GroupMemberAdd,
	create_group_memeber_add,
	GroupMemberAddType
);

#[derive(Debug, Clone)]
pub struct GroupMemberDecreaseType {
	/// 离开的用户id
	pub target_id: String,
	pub leave_type: GroupLeaveType,
}

#[derive(Debug, Clone)]
pub enum GroupLeaveType {
	/// 主动退群
	Leave,
	/// 群成员被踢
	Kick,
	/// Bot自身被踢
	KickBot,
}

impl_notion_event!(
	GroupMemberDecrease,
	"收到群成员减少事件",
	NotionSubEvent::GroupMemberDecrease,
	create_group_memeber_decrease,
	GroupMemberAddType
);

#[derive(Debug, Clone)]
pub struct GroupAdminChangeType {
	/// 被操作的id
	pub target_id: String,
	pub admin_type: GroupAdminType,
}

#[derive(Debug, Clone)]
pub enum GroupAdminType {
	/// 设置
	Set,
	/// 取消
	Remove,
}
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

#[derive(Debug, Clone)]
pub struct GroupMemberBanType {
	/// 被禁用的用户id
	pub target_id: String,
	/// 禁言时长(秒)
	pub duration: u64,
	pub ban_type: GroupBanType,
}

#[derive(Debug, Clone)]
pub enum GroupBanType {
	/// 禁言
	Ban,
	/// 解除禁言
	Unban,
}

impl_notion_event!(
	GroupMemberBan,
	"收到群成员禁言事件",
	NotionSubEvent::GroupMemberBan,
	create_group_member_ban,
	GroupMemberBanType
);

#[derive(Debug, Clone)]
pub struct GroupWholeBanType {
	pub action: GroupWholeBanActionType,
}

#[derive(Debug, Clone)]
pub enum GroupWholeBanActionType {
	/// 全员禁言
	WholeBan,
	/// 全员解除禁言
	WholeUnban,
}

impl_notion_event!(
	GroupWholeBan,
	"收到群全体成员禁言事件",
	NotionSubEvent::GroupWholeBan,
	create_group_whole_ban,
	GroupWholeBanType
);

#[derive(Debug, Clone)]
pub struct GroupMessageReactionType {
	pub message_id: String,
	pub face_id: u32,
	pub count: u8,
	pub action: GroupMessageReactionAction,
}

#[derive(Debug, Clone)]
pub enum GroupMessageReactionAction {
	/// 添加
	Add,
	/// 移除
	Remove,
}

impl_notion_event!(
	GroupMessageReaction,
	"收到群消息表情动态事件",
	NotionSubEvent::GroupMessageReaction,
	create_group_message_reaction,
	GroupMessageReactionType
);

#[derive(Debug, Clone)]
pub struct GroupLuckKingType {
	/// 运气王id
	pub target_id: String,
}

impl_notion_event!(
	GroupLuckKing,
	"收到群运气王事件",
	NotionSubEvent::GroupLuckKing,
	create_group_luck_king,
	GroupLuckKingType
);

#[derive(Debug, Clone)]
pub struct GroupHonorChangeType {
	pub honor_type: HonorType,
}

#[derive(Debug, Clone)]
pub enum HonorType {
	/// 龙王
	Talkative,
	/// 群聊之火
	Performer,
	/// 快乐源泉
	Emotion,
}
impl_notion_event!(
	GroupHonorChange,
	"收到群聊荣誉变更事件",
	NotionSubEvent::GroupHonorChange,
	create_group_honor_change,
	GroupHonorChangeType
);
