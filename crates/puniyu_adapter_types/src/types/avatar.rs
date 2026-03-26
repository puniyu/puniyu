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
pub struct Avatar<'a>(&'a Bytes);

impl<'a> Avatar<'a> {
    pub fn new(bytes: &'a Bytes) -> Self {
        Avatar(bytes)
    }

    pub fn bytes(&self) -> &Bytes {
        self.0
    }
}