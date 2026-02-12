/// 消息构建宏
///
/// 这个宏提供了一种便捷的方式来创建 `Message` 对象。
/// 它接受一个或多个 `Elements` 枚举值。
///
/// # 用法
///
/// ## 单个元素
/// 从单个消息元素创建消息：
/// ```rust
/// use puniyu_message::message;
/// use puniyu_element::send::{Elements, TextElement};
///
/// let msg = message!(Elements::Text(TextElement::new("Hello")));
/// ```
///
/// ## 多个元素
/// 从多个消息元素创建消息:
/// ```rust
/// use puniyu_message::message;
/// use puniyu_element::send::{Elements, TextElement, AtElement};
///
/// let msg = message!(
///     Elements::At(AtElement::new("123456")),
///     Elements::Text(TextElement::new("hello")),
/// );
/// ```
///
#[macro_export]
macro_rules! message {
    ($($element:expr),* $(,)?) => {
        $crate::Message::from(vec![$($element),*])
    };
}
