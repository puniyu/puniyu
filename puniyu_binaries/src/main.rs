use std::{
	io,
	net::{IpAddr, Ipv4Addr},
};

use convert_case::{Case, Casing};
use figlet_rs::FIGlet;
use semver::Version;
use tokio::signal;
mod log;

const NAME: &str = "puniyu";
const VERSION: Version = puniyu_version::VERSION;

#[tokio::main]
async fn main() -> io::Result<()> {
	log::init();
	let host = Ipv4Addr::new(127, 0, 0, 1);
	puniyu_server::App::hoop(puniyu_server_middleware::AccessLog);
	let server = tokio::spawn(puniyu_server::start_server(IpAddr::V4(host), 10721));
	signal::ctrl_c().await?;
	server.await?
}

fn print_start_log() {
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