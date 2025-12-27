mod friend;

pub use friend::*;
use serde::{Deserialize, Serialize};
mod group;
pub use group::*;

use super::EventBase;
use crate::bot::BotInfo;
use crate::contact::{FriendContact, GroupContact, Scene};
use crate::event::EventType;
use crate::sender::{FriendSender, GroupSender};
use strum::{Display, EnumString, IntoStaticStr};

#[derive(Debug, Clone, EnumString, Display, IntoStaticStr, Deserialize, Serialize)]
pub enum RequestSubEvent {
	/// 好友申请
	#[strum(serialize = "privateApply")]
	PrivateApply,
	/// 群申请
	#[strum(serialize = "groupApply")]
	GroupApply,
	/// 邀请入群
	#[strum(serialize = "groupInvite")]
	GroupInvite,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", tag = "type", content = "field0")]
pub enum RequestEvent {
	/// 好友申请
	PrivateApply(PrivateApply),
	/// 群申请
	GroupApply(GroupApply),
	/// 邀请入群
	GroupInvite(GroupInvite),
}

pub trait RequestBase: Send + Sync + EventBase {
	type Content;
	/// 请求消息
	fn notion(&self) -> &str;

	/// 请求内容
	fn content(&self) -> Self::Content;
}

#[derive(Debug, Clone)]
pub struct RequestBuilder<Contact, Sender> {
	pub bot: BotInfo,
	pub event_id: String,
	pub time: u64,
	pub self_id: String,
	pub user_id: String,
	pub contact: Contact,
	pub sender: Sender,
}

macro_rules! impl_request_event {
    (
        $(
            $(#[$attr:meta])*
            $struct_name:ident,
            $notion_desc:expr,
            $sub_event:expr,
            $contact_ty:ty,
            $sender_ty:ty,
            $content_ty:ty;
        )+
    ) => {
        $(
            impl_request_event!(
                $(#[$attr])*
                $struct_name,
                $notion_desc,
                $sub_event,
                $contact_ty,
                $sender_ty,
                $content_ty
            );
        )+
    };

    (
        $(#[$attr:meta])*
        $struct_name:ident,
        $notion_desc:expr,
        $sub_event:expr,
        $contact_ty:ty,
        $sender_ty:ty,
        $content_ty:ty
    ) => {
        $(#[$attr])*
        #[derive(Debug, Clone, Deserialize, Serialize)]
        pub struct $struct_name {
            bot: BotInfo,
            event_id: String,
            time: u64,
            self_id: String,
            user_id: String,
            contact: $contact_ty,
            sender: $sender_ty,
            content: $content_ty,
        }

        impl $struct_name {
            pub fn new(
                request_builder: RequestBuilder<$contact_ty, $sender_ty>,
                content: $content_ty
            ) -> Self {
                Self {
                    bot: request_builder.bot,
                    event_id: request_builder.event_id,
                    time: request_builder.time,
                    self_id: request_builder.self_id,
                    user_id: request_builder.user_id,
                    contact: request_builder.contact,
                    sender: request_builder.sender,
                    content,
                }
            }
        }

        impl EventBase for $struct_name {
            type ContactType = $contact_ty;
            type SenderType = $sender_ty;

            fn bot(&self) -> &BotInfo { &self.bot }
            fn time(&self) -> u64 { self.time }
            fn event(&self) -> &str { EventType::Request.into() }
            fn event_id(&self) -> &str { &self.event_id }
            fn sub_event(&self) -> &str { $sub_event.into() }
            fn self_id(&self) -> &str { &self.self_id }
            fn user_id(&self) -> &str { &self.user_id }
            fn contact(&self) -> Self::ContactType { self.contact.clone() }
            fn sender(&self) -> Self::SenderType { self.sender.clone() }
            fn is_friend(&self) -> bool { matches!(self.contact.scene, Scene::Friend) }
            fn is_group(&self) -> bool { matches!(self.contact.scene, Scene::Group) }
        }

        impl RequestBase for $struct_name {
            type Content = $content_ty;
            fn notion(&self) -> &str { $notion_desc }
            fn content(&self) -> Self::Content { self.content.clone() }
        }
    };
}

impl_request_event!(
	PrivateApply, "收到好友申请请求", RequestSubEvent::PrivateApply, FriendContact, FriendSender, PrivateApplyType;
	GroupApply, "收到入群申请请求", RequestSubEvent::GroupApply, GroupContact, GroupSender, GroupApplyType;
	GroupInvite, "收到群邀请请求", RequestSubEvent::GroupInvite, GroupContact, GroupSender, GroupInviteType;
);

#[cfg(feature = "event")]
#[macro_export]
macro_rules! create_request_event {
    (
        $variant:ident,
        $bot:ident,
        $( $key:ident : $value:expr ),* $(,)?
    ) => {{
        let mut builder = RequestBuilder {
            bot: Default::default(),
            event_id: String::new(),
            time: 0,
            self_id: String::new(),
            user_id: String::new(),
            contact: Default::default(),
            sender: Default::default(),
        };

        $(
            builder.$key = create_request_event!(@convert $key, $value);
        )*

        let request = $variant::new(builder);
        let event = Event::Request(RequestEvent::$variant(request));

        send_event($bot.clone(), event);
    }};

    (@convert adapter, $v:expr) => { $v };
    (@convert event_id, $v:expr) => { $v.to_string() };
    (@convert time, $v:expr) => { $v };
    (@convert self_id, $v:expr) => { $v.to_string() };
    (@convert user_id, $v:expr) => { $v.to_string() };
    (@convert contact, $v:expr) => { $v };
    (@convert sender, $v:expr) => { $v };
}
