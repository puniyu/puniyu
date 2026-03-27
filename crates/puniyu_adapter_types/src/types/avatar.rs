use bytes::Bytes;

/// 头像尺寸。
#[derive(Debug, Clone)]
pub enum AvatarSize {
	/// 小尺寸头像。
	Small,
	/// 中等尺寸头像。
	Medium,
	/// 大尺寸头像。
	Large,
}

/// 头像字节视图。
#[derive(Debug, Clone)]
pub struct Avatar<'a>(&'a Bytes);

impl<'a> Avatar<'a> {
	/// 创建头像视图。
	pub fn new(bytes: &'a Bytes) -> Self {
		Avatar(bytes)
	}

	/// 获取头像字节引用。
	pub fn bytes(&self) -> &Bytes {
		self.0
	}
}
