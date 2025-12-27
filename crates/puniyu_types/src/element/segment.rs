#[cfg(feature = "element")]
#[macro_export]
macro_rules! segment {
    // 提及元素
    (at, $target_id:expr) => {
        $crate::element::send::Elements::At($crate::element::send::AtElement {
            target_id: $target_id.to_string(),
        })
    };
    (at_all) => {
        $crate::element::send::Elements::At($crate::element::send::AtElement {
            target_id: String::from("all"),
        })
    };

    // 文本元素
    (text, $text:expr) => {
        $crate::element::send::Elements::Text($crate::element::send::TextElement {
            text: $text.to_string(),
        })
    };

    // 图片元素
    (image, $file:expr) => {
        segment!(image, $file, false, None)
    };
    (image, $file:expr, $is_flash:expr) => {
        segment!(image, $file, $is_flash, None)
    };
    (image, $file:expr, $is_flash:expr, $summary:expr) => {
        $crate::element::send::Elements::Image($crate::element::send::ImageElement {
            file: $file,
            is_flash: $is_flash,
            summary: $summary,
        })
    };

    // 回复元素
    (reply, $message_id:expr) => {
        $crate::element::send::Elements::Reply($crate::element::send::ReplyElement {
            message_id: $message_id,
        })
    };

    // 文件元素
    (file, $file:expr) => {
        $crate::element::send::Elements::File($crate::element::send::FileElement {
            file: $file,
        })
    };

    // 语音元素
    (record, $file:expr) => {
        $crate::element::send::Elements::Record($crate::element::send::RecordElement {
            file: $file,
        })
    };

    // 视频元素
    (video, $file:expr) => {
        $crate::element::send::Elements::Video($crate::element::send::VideoElement {
            file: $file,
        })
    };

    // 表情元素
    (face, $id:expr) => {
        $crate::element::send::Elements::Face($crate::element::send::FaceElement {
            id: $id,
        })
    };

    // JSON 元素
    (json, $json:expr) => {
        $crate::element::send::Elements::Json($crate::element::send::JsonElement {
            data: $json.to_string(),
        })
    };

    // XML 元素
    (xml, $xml:expr) => {
        $crate::element::send::Elements::Xml($crate::element::send::XmlElement {
            file: $xml.to_string(),
        })
    };
    ($($t:tt)*) => {
        compile_error!(concat!("Invalid segment! pattern: ", stringify!($($t)*)));
    };
}


#[macro_export]
macro_rules! message {
    // 单个参数
    ($single:expr) => {
        $crate::element::Message::from($single)
    };
    // 多个 Elements
    ($first:expr, $($rest:expr),+ $(,)?) => {
        $crate::element::Message::from(vec![$first, $($rest),+])
    };
}
