#[cfg(feature = "server")]
mod server;

use bon::Builder;
use semver::Version;
use std::time::Instant;
use std::io;


#[derive(Builder)]
pub struct App {
	#[builder(field)]
	handlers: Vec<Box<dyn puniyu_handler::Handler>>,
	#[builder(field)]
	#[cfg(feature = "server")]
	routers: Vec<salvo::Router>,
	#[builder(field)]
	#[cfg(feature = "server")]
	hoops: Vec<server::ArcHandler>,
	app_name: &'static str,
	app_version: Version,
}

impl Default for App {
	fn default() -> Self {
		Self {
			handlers: Vec::new(),
			routers: Vec::new(),
			hoops: Vec::new(),
			app_name: "",
			app_version: puniyu_version::VERSION,
		}
	}
}

impl<S: app_builder::State> AppBuilder<S> {
	pub fn handler(mut self, handler: impl puniyu_handler::Handler + 'static) -> Self {
		self.handlers.push(Box::new(handler));
		self
	}

	#[cfg(feature = "server")]
	pub fn router(mut self, router: salvo::Router) -> Self {
		self.routers.push(router);
		self
	}

	#[cfg(feature = "server")]
	pub fn hoop(mut self, hoop: impl salvo::Handler + 'static) -> Self {
		self.hoops.push(server::ArcHandler::new(hoop));
		self
	}
}

impl App {
	pub async fn run(self) -> io::Result<()> {
		let start_time = Instant::now();
		let cwd_dir = Box::leak(Box::new(std::env::current_dir()?));
		puniyu_app::App::init(self.app_name, cwd_dir);
		#[cfg(feature = "server")]
		{
			self.routers.into_iter().for_each(puniyu_server::App::router);
			self.hoops.into_iter().for_each(puniyu_server::App::hoop);
		}

		Ok(())
	}
}
