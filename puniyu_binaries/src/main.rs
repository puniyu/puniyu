mod log;
mod load;

use std::io;

use semver::Version;

pub(crate) const NAME: &str = "puniyu";
pub(crate) const VERSION: Version = puniyu_version::VERSION;

#[tokio::main]
async fn main() -> io::Result<()> {
	puniyu::App::builder()
		.name(NAME)
		.hoop(puniyu_server_middleware::AccessLog)
		.on_start(load::load)
		.build()
		.run()
		.await
}


