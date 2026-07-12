/// 快速构建消息元素。
///
/// 常用写法：
///
/// ```rust
/// use puniyu_segment::segment;
/// use puniyu_element::File;
/// use bytes::Bytes;
///
/// let text = segment!(text, "Hello");
/// let at = segment!(at, "123456");
/// let at_all = segment!(at, all);
/// let face = segment!(face, 123u64);
/// let image = segment!(image, File::Bytes(Bytes::from_static(b"...")), "image.png", Some("summary"));
/// ```
#[macro_export]
macro_rules! segment {
    (at, all) => {
        $crate::at("all")
    };
    (at, $target_id:expr) => {
        $crate::at($target_id)
    };
    (text, $text:expr) => {
        $crate::text($text)
    };
    (image, $file:expr, $file_name:expr) => {
        $crate::image($file, $file_name, None)
    };
    (image, $file:expr, $file_name:expr, $summary:expr) => {
        $crate::image($file, $file_name, Some($summary))
    };
    (reply, $message_id:expr) => {
        $crate::reply($message_id)
    };
    (file, $file:expr, $file_name:expr) => {
        $crate::file_seg($file, $file_name)
    };
    (record, $file:expr, $file_name:expr) => {
        $crate::record($file, $file_name)
    };
    (video, $file:expr, $file_name:expr) => {
        $crate::video($file, $file_name)
    };
    (face, $id:expr) => {
        $crate::face($id)
    };
    (json, $data:expr) => {
        $crate::json($data)
    };
    (xml, $data:expr) => {
        $crate::xml($data)
    };
    ($($t:tt)*) => {
        compile_error!(concat!("Invalid segment! macro pattern: ", stringify!($($t)*)));
    };
}
