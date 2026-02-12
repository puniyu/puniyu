use bytes::Bytes;
use puniyu_server::{load_logo_from_file, set_logo};
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_set_logo() {
    let logo_data = Bytes::from(vec![0x89, 0x50, 0x4E, 0x47]); // PNG header
    set_logo(logo_data.clone());

}

#[test]
fn test_set_empty_logo() {
    let empty_data = Bytes::new();
    set_logo(empty_data);
}

#[test]
fn test_load_logo_from_file_success() {
    let mut temp_file = NamedTempFile::new().unwrap();
    let logo_data = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    temp_file.write_all(&logo_data).unwrap();
    
    let result = load_logo_from_file(temp_file.path());
    assert!(result.is_ok());
}

#[test]
fn test_load_logo_from_nonexistent_file() {
    let result = load_logo_from_file("nonexistent_file.png");
    assert!(result.is_err());
}

#[test]
fn test_load_logo_from_empty_file() {
    let temp_file = NamedTempFile::new().unwrap();
    
    // 加载空文件
    let result = load_logo_from_file(temp_file.path());
    assert!(result.is_ok());
}

#[test]
fn test_multiple_logo_updates() {
    let logo1 = Bytes::from(vec![1, 2, 3]);
    let logo2 = Bytes::from(vec![4, 5, 6]);
    let logo3 = Bytes::from(vec![7, 8, 9]);
    
    set_logo(logo1);
    set_logo(logo2);
    set_logo(logo3);

}
