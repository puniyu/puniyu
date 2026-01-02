use puniyu_adapter::{AccountApi, FriendApi, GroupApi, MessageApi};

pub struct ServerMessageApi;

impl MessageApi for ServerMessageApi {}

pub struct ServerAccountApi;

impl AccountApi for ServerAccountApi {}

pub struct ServerFriendApi;

impl FriendApi for ServerFriendApi {}

pub struct ServerGroupApi;
impl GroupApi for ServerGroupApi {}