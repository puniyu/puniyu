use puniyu_logger::LoggerOptions;

pub fn init() {
	let options = LoggerOptions::default();

	puniyu_logger::init(Some(options));
}
