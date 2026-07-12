use std::path::{Path, PathBuf};

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

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

#[derive(
	Debug,
	Clone,
	PartialEq,
	Eq,
	Hash,
	EnumString,
	Display,
	IntoStaticStr,
	Deserialize,
	Serialize,
)]
#[serde(rename_all = "lowercase", tag = "type", content = "field0")]
#[strum(serialize_all = "lowercase")]
pub enum File {
	Bytes(Bytes),
	Path(PathBuf),
}

impl From<Bytes> for File {
	fn from(value: Bytes) -> Self {
		Self::Bytes(value)
	}
}

impl From<PathBuf> for File {
	fn from(value: PathBuf) -> Self {
		Self::Path(value)
	}
}

impl File {
	pub fn as_bytes(&self) -> Option<&Bytes> {
		match self {
			Self::Bytes(bytes) => Some(bytes),
			_ => None,
		}
	}
	pub fn as_path(&self) -> Option<&Path> {
		match self {
			Self::Path(path) => Some(path),
			_ => None,
		}
	}
}
