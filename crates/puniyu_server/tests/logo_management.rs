use bytes::Bytes;
use puniyu_server::set_logo;

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
fn test_multiple_logo_updates() {
    let logo1 = Bytes::from(vec![1, 2, 3]);
    let logo2 = Bytes::from(vec![4, 5, 6]);
    let logo3 = Bytes::from(vec![7, 8, 9]);
    
    set_logo(logo1);
    set_logo(logo2);
    set_logo(logo3);

}
