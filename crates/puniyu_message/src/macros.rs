/// 快速构建 [`Message`](crate::Message)。
///
/// 每个参数都可以是具体发送元素或已经包装的 `Elements`。
///
/// ```rust
/// use puniyu_element::send::{AtElement, TextElement};
/// use puniyu_message::message;
///
/// let msg = message!(AtElement::new("123456"), TextElement::new("hello"));
/// assert_eq!(msg.len(), 2);
/// ```
#[macro_export]
macro_rules! message {
    ($($element:expr),* $(,)?) => {
        $crate::Message::builder()
            $(.element($element))*
            .build()
    };
}
