use puniyu_handler::Handler;
use puniyu_handler_command::Handler as CommandHandler;

#[test]
fn command_handler_name_matches_expected() {
	let handler = CommandHandler;
	assert_eq!(handler.name(), "command");
	assert_eq!(handler.priority(), 5);
}
