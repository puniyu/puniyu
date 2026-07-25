use std::path::PathBuf;

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};
use url::Url;

use std::hash::Hash;

pub trait Element: Send + Sync {
	/// 元素类型
	fn r#type(&self) -> ElementType;
}

impl<T: Element + ?Sized> Element for &T {
	fn r#type(&self) -> ElementType {
		(**self).r#type()
	}
}

impl PartialEq for dyn Element {
	fn eq(&self, other: &Self) -> bool {
		self.r#type() == other.r#type()
	}
}

impl Eq for dyn Element {}

/// 元素类型枚举。
#[derive(
	Debug,
	Copy,
	Clone,
	Hash,
	PartialEq,
	Eq,
	EnumString,
	Display,
	IntoStaticStr,
	Deserialize,
	Serialize,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum ElementType {
	/// 艾特元素
	At,
	/// 回复元素
	Reply,
	/// 文本元素
	Text,
	/// 图片元素
	Image,
	/// 文件元素
	File,
	/// 语音元素
	Record,
	/// 视频元素
	Video,
	/// 表情元素
	Face,
	/// json元素
	Json,
	/// xml元素
	Xml,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", tag = "type", content = "field0")]
pub enum File {
	Bytes(Bytes),
	Url(Url),
}

impl From<Bytes> for File {
	fn from(value: Bytes) -> Self {
		Self::Bytes(value)
	}
}

impl From<PathBuf> for File {
	fn from(value: PathBuf) -> Self {
		match Url::from_file_path(&value) {
			Ok(url) => Self::Url(url),
			Err(()) => {
				let s = value.to_string_lossy();
				let url = Url::parse(&format!("file:///{}", s)).expect("valid relative file url");
				Self::Url(url)
			}
		}
	}
}

impl File {
	pub fn as_bytes(&self) -> Option<&Bytes> {
		match self {
			Self::Bytes(bytes) => Some(bytes),
			_ => None,
		}
	}
	pub fn as_url(&self) -> Option<&Url> {
		match self {
			Self::Url(url) => Some(url),
			_ => None,
		}
	}


	pub fn as_path(&self) -> Option<PathBuf> {
		match self {
			Self::Url(url) if url.scheme() == "file" => url.to_file_path().ok(),
			_ => None,
		}
	}
}
