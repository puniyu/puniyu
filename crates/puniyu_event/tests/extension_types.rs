use puniyu_event::extension::ExtensionSubEventType;

#[test]
fn extension_sub_event_type_is_lightweight_and_stable() {
	let kind = ExtensionSubEventType::new("test.extension");
	let copied = kind;

	assert_eq!(kind, copied);
	assert_eq!(kind.kind(), "test.extension");
	assert_eq!(kind.to_string(), "test.extension");
}
