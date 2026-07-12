use convert_case::{Case, Casing};
use figlet_rs::FIGlet;
use crate::{NAME, VERSION};
use puniyu_logger::LoggerOptions;

pub fn init() {
	let options = LoggerOptions::default();

	puniyu_logger::init(Some(options));
}

pub fn start_log() {
	let app_name = NAME.to_case(Case::Lower);
	if let Ok(standard_font) = FIGlet::standard()
		&& let Some(art_text) = standard_font.convert(app_name.as_str())
	{
		println!("{}", art_text);
	} else {
		println!("{}", app_name);
	}

	println!("{} starting...", app_name.to_case(Case::Lower));
	println!("Version: {}", VERSION);
	println!("Git SHA: {}", env!("VERGEN_GIT_SHA"));
	println!("Github: {}", env!("CARGO_PKG_REPOSITORY"));
}
