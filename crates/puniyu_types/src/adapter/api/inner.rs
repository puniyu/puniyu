use super::{FriendApi, AccountApi, MessageApi};

pub(crate) struct EmptyFriendApi;

impl FriendApi for EmptyFriendApi {
}

pub(crate) struct EmptyAccountApi;

impl AccountApi for EmptyAccountApi {
}

pub(crate) struct EmptyMessageApi;

impl MessageApi for EmptyMessageApi {
}
