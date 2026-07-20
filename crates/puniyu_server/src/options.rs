use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct ServerOptions {
	pub host: IpAddr,
	pub port: u16,
	pub shutdown_timeout: Duration,
}

impl Default for ServerOptions {
	fn default() -> Self {
		Self {
			host: IpAddr::V4(Ipv4Addr::LOCALHOST),
			port: 10721,
			shutdown_timeout: Duration::from_secs(10),
		}
	}
}
