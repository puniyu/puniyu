/// 用于快速构建消息元素的宏
///
/// 该宏通过 `Segment` 结构体的方法来构建各种消息元素，提供了简洁的语法糖。
///
/// # 支持的元素类型
///
/// ## At 元素
/// - `segment!(at, "all")` - @全体成员
/// - `segment!(at, "123456")` - @指定用户
///
/// ## 文本元素
/// - `segment!(text, "Hello")` - 纯文本消息
///
/// ## 图片元素
/// - `segment!(image, file_bytes, "image.jpg")` - 图片，使用文件名作为外显
/// - `segment!(image, file_bytes, "image.jpg", "Image description")` - 图片，指定外显文本
///
/// ## 回复元素
/// - `segment!(reply, "message_id")` - 回复指定消息
///
/// ## 文件元素
/// - `segment!(file, file_bytes, "file.txt")` - 发送文件
///
/// ## 语音元素
/// - `segment!(record, audio_bytes, "audio.mp3")` - 发送语音
///
/// ## 视频元素
/// - `segment!(video, video_bytes, "video.mp4")` - 发送视频
///
/// ## 表情元素
/// - `segment!(face, 123u64)` - 发送表情，使用表情 ID
///
/// ## JSON 元素
/// - `segment!(json, r#"{"key": "value"}"#)` - 发送 JSON 数据
///
/// ## XML 元素
/// - `segment!(xml, r#"<xml>...</xml>"#)` - 发送 XML 数据
///
/// # 示例
///
/// ```rust
/// use puniyu_segment::segment;
/// use bytes::Bytes;
///
/// // 创建文本消息
/// let text = segment!(text, "Hello, World!");
///
/// // @某人
/// let at = segment!(at, "123456");
///
/// // 发送图片
/// let image_data = Bytes::from("image data");
/// let image = segment!(image, image_data, "photo.jpg", "Beautiful scenery");
/// ```
#[macro_export]
macro_rules! segment {
    // At 元素 - @全体成员
    (at, all) => {
        segment!(at, "all")
    };
    // At 元素 - @指定用户
    (at, $target_id:expr) => {
        $crate::Segment::at($target_id)
    };
    // 文本元素
    (text, $text:expr) => {
        $crate::Segment::text($text)
    };
    // 图片元素 - 使用文件名作为外显
    (image, $file:expr, $file_name:expr) => {
        $crate::Segment::image($file, $file_name, None)
    };
    // 图片元素 - 指定外显文本
    (image, $file:expr, $file_name:expr, $summary:expr) => {
        $crate::Segment::image($file, $file_name, Some($summary))
    };
    // 回复元素
    (reply, $message_id:expr) => {
        $crate::Segment::reply($message_id)
    };
    // 文件元素
    (file, $file:expr, $file_name:expr) => {
        $crate::Segment::file($file, $file_name)
    };
    // 语音元素
    (record, $file:expr, $file_name:expr) => {
        $crate::Segment::record($file, $file_name)
    };
    // 视频元素
    (video, $file:expr, $file_name:expr) => {
        $crate::Segment::video($file, $file_name)
    };
    // 表情元素
    (face, $id:expr) => {
        $crate::Segment::face($id)
    };
    // JSON 元素
    (json, $data:expr) => {
        $crate::Segment::json($data)
    };
    // XML 元素
    (xml, $data:expr) => {
        $crate::Segment::xml($data)
    };
    // 错误处理 - 无效的模式
    ($($t:tt)*) => {
        compile_error!(concat!("无效的 segment! 宏模式: ", stringify!($($t)*)));
    };
}
