//! # puniyu_element
//!
//! 消息元素类型定义库，为 Puniyu 框架提供接收和发送消息元素的类型系统。
//!
//! ## 概述
//!
//! 本库提供了一套完整的消息元素类型定义，分为两类：
//!
//! - **接收元素（receive）** - 从聊天平台接收到的消息元素
//! - **发送元素（send）** - 向聊天平台发送的消息元素
//!
//! ## 使用示例
//!
//! ### 接收消息元素
//!
//! ```rust
//! use puniyu_element::receive::*;
//! use puniyu_element::RawMessage;
//!
//! // 创建文本元素
//! let text = TextElement { text: "Hello, World!" };
//! println!("Message: {}", text.raw());
//!
//! // 创建 @提及元素
//! let at = AtElement { target_id: "123456" };
//! if at.is_everyone() {
//!     println!("@Everyone");
//! } else {
//!     println!("@User: {}", at.target_id);
//! }
//!
//! // 使用元素枚举
//! let element = Elements::Text(TextElement { text: "Test message" });
//! if let Some(text) = element.as_text() {
//!     println!("Text content: {}", text);
//! }
//! ```
//!
//! ### 发送消息元素
//!
//! ```rust
//! use puniyu_element::send::*;
//! use bytes::Bytes;
//!
//! // 创建文本元素
//! let text = TextElement::new("Hello!");
//!
//! // 创建图片元素
//! let image_data = Bytes::from("image data");
//! let image = ImageElement::new(image_data, "photo.jpg", None);
//!
//! // 创建表情元素
//! let face = FaceElement::new(42u64);
//! ```
//!
//! ## 支持的元素类型
//!
//! - `TextElement` - 纯文本消息
//! - `AtElement` - @提及用户
//! - `ReplyElement` - 回复消息
//! - `FaceElement` - 表情符号
//! - `ImageElement` - 图片
//! - `FileElement` - 文件
//! - `VideoElement` - 视频
//! - `RecordElement` - 语音/音频
//! - `JsonElement` - JSON 数据
//! - `XmlElement` - XML 数据

pub mod receive;
pub mod send;
mod types;
#[doc(inline)]
pub use types::{ElementType, RawMessage, Element};


macro_rules! codegen_reexport {
    ($($module:ident => $type:ident),*) => {
        $(
            mod $module;
			#[doc(inline)]
            pub use $module::$type;
        )*
    };
}
pub(crate) use codegen_reexport;