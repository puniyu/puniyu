use salvo::conn::TcpListener;
use salvo::prelude::*;
use salvo::server::ServerHandle;

use crate::registry::ServerRegistry;
use std::io;
use std::net::{IpAddr, SocketAddr};
use std::sync::{LazyLock, Mutex};

macro_rules! info {
    ($($arg:tt)*) => {
        {
            {
                use ::puniyu_logger::owo_colors::OwoColorize;
                let prefix = "Server".fg_rgb::<132,112,255>();
                ::log::info!("[{}] {}", prefix, format!($($arg)*))
            }
        }
    };
}

static SERVER_HANDLE: LazyLock<Mutex<Option<ServerHandle>>> = LazyLock::new(|| Mutex::new(None));

pub async fn start_server(host: IpAddr, port: u16) -> io::Result<()> {
	{
		let guard = SERVER_HANDLE.lock().map_err(|e| io::Error::other(e.to_string()))?;
		if guard.is_some() {
			return Err(io::Error::new(io::ErrorKind::AlreadyExists, "Server already running"));
		}
	}

	info!("Server running on {}:{}", host, port);

	let listener = TcpListener::new(SocketAddr::new(host, port)).bind().await;
	let inner = crate::app::take();
	let mut router = Router::new();
	router = inner.routers.into_iter().fold(router, Router::push);
	for info in ServerRegistry::all() {
		router = router.push(info.router.take());
	}
	let mut service = Service::new(router);
	service.hoops.extend(inner.hoops);

	let server = Server::new(listener);
	let handle = server.handle();

	SERVER_HANDLE.lock().map_err(|e| io::Error::other(e.to_string()))?.replace(handle);

	server.serve(service).await;
	Ok(())
}

pub async fn stop_server() -> io::Result<()> {
	let handle = SERVER_HANDLE
		.lock()
		.map_err(|e| io::Error::other(e.to_string()))?
		.take()
		.ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Server not running"))?;

	handle.stop_forceful();
	Ok(())
}
