use super::{AccountApi, FriendApi, MessageApi};
use crate::adapter::GroupApi;

pub(crate) struct DefaultGroupApi;

impl GroupApi for DefaultGroupApi {}

pub(crate) struct DefaultFriendApi;

impl FriendApi for DefaultFriendApi {}

pub(crate) struct DefaultAccountApi;

impl AccountApi for DefaultAccountApi {}

pub(crate) struct DefaultMessageApi;

impl MessageApi for DefaultMessageApi {}
