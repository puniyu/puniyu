use puniyu_event::request::RequestSubEvent;

#[test]
fn test_request_sub_event_display() {
    assert_eq!(RequestSubEvent::PrivateApply.to_string(), "privateApply");
    assert_eq!(RequestSubEvent::GroupApply.to_string(), "groupApply");
    assert_eq!(RequestSubEvent::GroupInvite.to_string(), "groupInvite");
}

#[test]
fn test_request_sub_event_from_str() {
    use std::str::FromStr;
    
    assert_eq!(
        RequestSubEvent::from_str("privateApply").unwrap(),
        RequestSubEvent::PrivateApply
    );
    assert_eq!(
        RequestSubEvent::from_str("groupApply").unwrap(),
        RequestSubEvent::GroupApply
    );
    assert_eq!(
        RequestSubEvent::from_str("groupInvite").unwrap(),
        RequestSubEvent::GroupInvite
    );
}

#[test]
fn test_request_sub_event_equality() {
    assert_eq!(RequestSubEvent::PrivateApply, RequestSubEvent::PrivateApply);
    assert_eq!(RequestSubEvent::GroupApply, RequestSubEvent::GroupApply);
    assert_ne!(RequestSubEvent::PrivateApply, RequestSubEvent::GroupApply);
}

#[test]
fn test_request_sub_event_ordering() {
    assert!(RequestSubEvent::PrivateApply < RequestSubEvent::GroupApply);
    assert!(RequestSubEvent::GroupApply < RequestSubEvent::GroupInvite);
}

#[test]
fn test_request_sub_event_clone() {
    let event = RequestSubEvent::PrivateApply;
    let cloned = event.clone();
    assert_eq!(event, cloned);
}
