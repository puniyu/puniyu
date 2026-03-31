#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
	#[error("not implemented")]
	NotImpl,
}

mod account;
pub(crate) use account::DefaultAccountApi;
mod group;
pub(crate) use group::DefaultGroupApi;
mod message;
pub(crate) use message::DefaultMessageApi;
mod friend;
pub(crate) use friend::DefaultFriendApi;
