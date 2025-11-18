use puniyu_element::*;

#[test]
fn test_text() {
    let elem = element!(text, "Hello");
    assert_eq!(elem.as_text(), Some("Hello"));
    assert_eq!(elem.raw(), "[text:Hello]");
}

#[test]
fn test_at() {
    let elem = element!(at, "123456");
    match elem {
        Elements::At(at) => {
            assert_eq!(at.target_id, "123456");
            assert!(!at.is_all());
        }
        _ => panic!("Expected At element"),
    }
}

#[test]
fn test_at_all() {
    let elem = element!(at_all);
    match elem {
        Elements::At(at) => {
            assert_eq!(at.target_id, "all");
            assert!(at.is_all());
        }
        _ => panic!("Expected At element"),
    }
}

#[test]
fn test_reply() {
    let elem = element!(reply, "msg_123");
    match &elem {
        Elements::Reply(reply) => assert_eq!(reply.message_id, "msg_123"),
        _ => panic!("Expected Reply element"),
    }
    assert_eq!(elem.raw(), "[reply:msg_123]");
}

#[test]
fn test_image() {
    let data = vec![1, 2, 3];
    let elem = element!(image, data.clone());
    match elem {
        Elements::Image(img) => {
            assert_eq!(img.file, data);
            assert!(!img.is_flash);
            assert_eq!(img.summary, None);
        }
        _ => panic!("Expected Image element"),
    }
}

#[test]
fn test_image_flash() {
    let data = vec![1, 2, 3];
    let elem = element!(image, data.clone(), true);
    match elem {
        Elements::Image(img) => {
            assert_eq!(img.file, data);
            assert!(img.is_flash);
        }
        _ => panic!("Expected Image element"),
    }
}

#[test]
fn test_image_summary() {
    let data = vec![1, 2, 3];
    let elem = element!(image, data.clone(), false, "Photo");
    match &elem {
        Elements::Image(img) => {
            assert_eq!(img.file, data);
            assert!(!img.is_flash);
            assert_eq!(img.summary, Some("Photo".to_string()));
        }
        _ => panic!("Expected Image element"),
    }
    assert_eq!(elem.raw(), "[image:Photo]");
}

#[test]
fn test_record() {
    let data = vec![1, 2, 3];
    let elem = element!(record, data.clone());
    match &elem {
        Elements::Record(rec) => assert_eq!(rec.file, data),
        _ => panic!("Expected Record element"),
    }
    assert_eq!(elem.raw(), "[record:语音消息]");
}

#[test]
fn test_file() {
    let data = vec![1, 2, 3];
    let elem = element!(file, data.clone(), "file_id", 1024, "test.pdf");
    match &elem {
        Elements::File(f) => {
            assert_eq!(f.file, data);
            assert_eq!(f.file_id, "file_id");
            assert_eq!(f.file_size, 1024);
            assert_eq!(f.file_name, "test.pdf");
        }
        _ => panic!("Expected File element"),
    }
    assert_eq!(elem.raw(), "[file:file_id]");
}

#[test]
fn test_video() {
    let data = vec![1, 2, 3];
    let elem = element!(video, data.clone(), "video.mp4");
    match &elem {
        Elements::Video(v) => {
            assert_eq!(v.file, data);
            assert_eq!(v.file_name, "video.mp4");
        }
        _ => panic!("Expected Video element"),
    }
    assert_eq!(elem.raw(), "[video:video.mp4]");
}

#[test]
fn test_json() {
    let elem = element!(json, "test");
    match elem {
        Elements::Json(j) => {
            assert!(j.data.is_string());
        }
        _ => panic!("Expected Json element"),
    }
}

#[test]
fn test_xml() {
    let elem = element!(xml, "<root>data</root>");
    match &elem {
        Elements::Xml(x) => assert_eq!(x.data, "<root>data</root>"),
        _ => panic!("Expected Xml element"),
    }
    assert_eq!(elem.raw(), "[xml:<root>data</root>]");
}

#[test]
fn test_as_text_none() {
    let elem = element!(at, "123");
    assert_eq!(elem.as_text(), None);
}

#[test]
fn test_vec_raw() {
    let elements = vec![
        element!(text, "Hello "),
        element!(at, "123"),
        element!(text, " World"),
    ];
    assert_eq!(elements.raw(), "[text:Hello ][at:123][text: World]");
}

#[test]
fn test_clone() {
    let elem = element!(text, "Test");
    let cloned = elem.clone();
    assert_eq!(elem.as_text(), cloned.as_text());
}

#[test]
fn test_serialize() {
    let elem = element!(text, "Hello");
    let json = serde_json::to_string(&elem).unwrap();
    assert!(json.contains("Hello"));
}

#[test]
fn test_deserialize() {
    let json = r#"{"Text":{"text":"Hello"}}"#;
    let elem: Elements = serde_json::from_str(json).unwrap();
    assert_eq!(elem.as_text(), Some("Hello"));
}

#[test]
fn test_round_trip() {
    let original = element!(text, "Test");
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: Elements = serde_json::from_str(&json).unwrap();
    assert_eq!(original.as_text(), deserialized.as_text());
}

#[test]
fn test_at_with_name() {
    let elem = Elements::At(AtElement {
        target_id: "123".to_string(),
        name: Some("User".to_string()),
    });
    match elem {
        Elements::At(at) => {
            assert_eq!(at.name, Some("User".to_string()));
        }
        _ => panic!("Expected At element"),
    }
}

#[test]
fn test_multiple_elements() {
    let elements = [
        element!(text, "Test"),
        element!(at, "user1"),
        element!(image, vec![1, 2, 3]),
        element!(reply, "msg1"),
    ];
    assert_eq!(elements.len(), 4);
}

#[test]
fn test_empty_vec() {
    let elements: Vec<Elements> = vec![];
    assert_eq!(elements.raw(), "");
}

#[test]
fn test_text_message_trait() {
    let elem = TextElement {
        text: "Hello".to_string(),
    };
    assert_eq!(elem.text(), "Hello");
}
