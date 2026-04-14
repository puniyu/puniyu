pub mod event;
#[doc(inline)]
pub use event::MessageEvent;

pub mod friend;
#[doc(inline)]
pub use friend::{FriendMessageReceive, FriendMessageSend};

pub mod group;
#[doc(inline)]
pub use group::{GroupMessageReceive, GroupMessageSend};
#[doc(inline)]
pub use group::temp::{GroupTempMessageReceive, GroupTempMessageSend};

pub mod guild;
#[doc(inline)]
pub use guild::{GuildMessageReceive, GuildMessageSend};
