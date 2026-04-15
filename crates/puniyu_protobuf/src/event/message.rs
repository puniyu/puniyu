pub mod event;
#[doc(inline)]
pub use event::MessageEvent;

pub mod friend;
#[doc(inline)]
pub use friend::FriendMessage;

pub mod group;
#[doc(inline)]
pub use group::GroupMessage;
#[doc(inline)]
pub use group::temp::GroupTempMessage;

pub mod guild;
#[doc(inline)]
pub use guild::GuildMessage;
