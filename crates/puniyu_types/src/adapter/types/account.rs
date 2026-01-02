use bytes::Bytes;

#[derive(Debug, Clone)]
pub enum AvatarSize {
    /// 小尺寸头像
    Small,
    /// 中等尺寸头像
    Medium,
    /// 大尺寸头像
    Large,
}

#[derive(Debug, Clone)]
pub struct Avatar(Bytes);

impl Avatar {
    pub fn new(bytes: Bytes) -> Self {
        Avatar(bytes)
    }
    
    pub fn bytes(&self) -> &Bytes {
        &self.0
    }
}


#[derive(Debug, Clone)]
pub enum SetFriendApplyType {
    /// 同意好友申请
    Agree,
    /// 拒绝好友申请
    Refuse,
}