//! # puniyu_segment
//!
//! 消息段构建工具库，提供便捷的方法和宏来构建各种类型的消息元素。
//!
//! ## 使用方式
//!
//! 本库提供两种构建消息元素的方式：
//!
//! ### 1. 使用 `Segment` 结构体方法
//!
//! ```rust
//! use puniyu_segment::Segment;
//! use bytes::Bytes;
//!
//! // 文本消息
//! let text = Segment::text("Hello, World!");
//!
//! // @用户
//! let at = Segment::at("123456");
//!
//! // 表情
//! let face = Segment::face(123u64);
//!
//! // 图片
//! let image_data = Bytes::from("image data");
//! let image = Segment::image(image_data, "photo.jpg", None);
//! ```
//!
//! ### 2. 使用 `segment!` 宏（推荐）
//!
//! ```rust
//! use puniyu_segment::segment;
//! use bytes::Bytes;
//!
//! // 文本消息
//! let text = segment!(text, "Hello, World!");
//!
//! // @用户
//! let at = segment!(at, "123456");
//!
//! // @全体成员
//! let at_all = segment!(at, all);
//!
//! // 表情
//! let face = segment!(face, 123u64);
//! ```
//!
//! ## 组合使用
//!
//! ```rust
//! use puniyu_segment::segment;
//! use puniyu_element::send::Elements;
//!
//! // 构建消息链
//! let message: Vec<Elements> = vec![
//!     segment!(at, "123456"),
//!     segment!(text, " Hello!"),
//!     segment!(face, 178u64),
//! ];
//! ```

use bytes::Bytes;
use puniyu_element::send::*;

mod macros;

/// 消息段构建器
///
/// 提供了一组静态方法用于构建各种类型的消息元素。
/// 所有方法都返回 `Elements<'s>` 枚举，可以直接用于消息发送。
///
/// # 示例
///
/// ```rust
/// use puniyu_segment::Segment;
/// use bytes::Bytes;
///
/// // 创建文本消息
/// let text = Segment::text("Hello, World!");
///
/// // @某个用户
/// let at = Segment::at("123456");
///
/// // 发送图片
/// let image_data = Bytes::from("image data");
/// let image = Segment::image(image_data, "photo.jpg", Some("Beautiful scenery"));
/// ```
pub struct Segment;

impl<'s> Segment {
	/// 创建 At 元素
	///
	/// # 参数
	///
	/// * `target_id` - 目标用户 ID，使用 "all" 表示 @全体成员
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_segment::Segment;
	///
	/// // @指定用户
	/// let at_user = Segment::at("123456");
	///
	/// // @全体成员
	/// let at_all = Segment::at("all");
	/// ```
	pub fn at(target_id: &'s str) -> Elements<'s> {
		let elem = AtElement::new(target_id);
		Elements::At(elem)
	}

	/// 创建文本元素
	///
	/// # 参数
	///
	/// * `text` - 文本内容
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_segment::Segment;
	///
	/// let text = Segment::text("Hello, World!");
	/// ```
	pub fn text(text: &'s str) -> Elements<'s> {
		let elem = TextElement::new(text);
		Elements::Text(elem)
	}

	/// 创建表情元素
	///
	/// # 参数
	///
	/// * `id` - 表情 ID
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_segment::Segment;
	///
	/// let face = Segment::face(123u64);
	/// ```
	pub fn face(id: impl Into<u64>) -> Elements<'s> {
		let elem = FaceElement::new(id.into());
		Elements::Face(elem)
	}

	/// 创建回复元素
	///
	/// # 参数
	///
	/// * `message_id` - 要回复的消息 ID
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_segment::Segment;
	///
	/// let reply = Segment::reply("message_12345");
	/// ```
	pub fn reply(message_id: &'s str) -> Elements<'s> {
		let elem = ReplyElement::new(message_id);
		Elements::Reply(elem)
	}

	/// 创建图片元素
	///
	/// # 参数
	///
	/// * `image` - 图片数据（字节）
	/// * `name` - 图片文件名
	/// * `summary` - 图片外显文本，如果为 `None` 则使用文件名
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_segment::Segment;
	/// use bytes::Bytes;
	///
	/// let image_data = Bytes::from("image data");
	///
	/// // 使用文件名作为外显
	/// let image1 = Segment::image(image_data.clone(), "photo.jpg", None);
	///
	/// // 指定外显文本
	/// let image2 = Segment::image(image_data, "photo.jpg", Some("Beautiful scenery"));
	/// ```
	pub fn image(image: impl Into<Bytes>, name: &'s str, summary: Option<&'s str>) -> Elements<'s> {
		let elem = ImageElement::new(image.into(), name, summary);
		Elements::Image(elem)
	}

	/// 创建语音元素
	///
	/// # 参数
	///
	/// * `record` - 语音数据（字节）
	/// * `name` - 语音文件名
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_segment::Segment;
	/// use bytes::Bytes;
	///
	/// let audio_data = Bytes::from("audio data");
	/// let record = Segment::record(audio_data, "voice.mp3");
	/// ```
	pub fn record(record: impl Into<Bytes>, name: &'s str) -> Elements<'s> {
		let elem = RecordElement::new(record.into(), name);
		Elements::Record(elem)
	}

	/// 创建文件元素
	///
	/// # 参数
	///
	/// * `file` - 文件数据（字节）
	/// * `name` - 文件名
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_segment::Segment;
	/// use bytes::Bytes;
	///
	/// let file_data = Bytes::from("file content");
	/// let file = Segment::file(file_data, "document.pdf");
	/// ```
	pub fn file(file: impl Into<Bytes>, name: &'s str) -> Elements<'s> {
		let elem = FileElement::new(file.into(), name);
		Elements::File(elem)
	}

	/// 创建视频元素
	///
	/// # 参数
	///
	/// * `video` - 视频数据（字节）
	/// * `name` - 视频文件名
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_segment::Segment;
	/// use bytes::Bytes;
	///
	/// let video_data = Bytes::from("video content");
	/// let video = Segment::video(video_data, "clip.mp4");
	/// ```
	pub fn video(video: impl Into<Bytes>, name: &'s str) -> Elements<'s> {
		let elem = VideoElement::new(video.into(), name);
		Elements::Video(elem)
	}

	/// 创建 JSON 元素
	///
	/// # 参数
	///
	/// * `json` - JSON 字符串数据
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_segment::Segment;
	///
	/// let json = Segment::json(r#"{"key": "value"}"#);
	/// ```
	pub fn json(json: &'s str) -> Elements<'s> {
		let elem = JsonElement::new(json);
		Elements::Json(elem)
	}

	/// 创建 XML 元素
	///
	/// # 参数
	///
	/// * `xml` - XML 字符串数据
	///
	/// # 示例
	///
	/// ```rust
	/// use puniyu_segment::Segment;
	///
	/// let xml = Segment::xml(r#"<xml><item>value</item></xml>"#);
	/// ```
	pub fn xml(xml: &'s str) -> Elements<'s> {
		let elem = XmlElement::new(xml);
		Elements::Xml(elem)
	}
}
