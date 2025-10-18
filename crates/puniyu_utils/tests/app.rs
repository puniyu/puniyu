use puniyu_utils::APP_NAME;
#[test]
fn name() {
	APP_NAME.get_or_init(|| String::from("puniyu"));

	assert_eq!(APP_NAME.get().unwrap(), "puniyu");
}
