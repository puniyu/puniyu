use async_trait::async_trait;
use puniyu_api::{pkg_name, pkg_version};
use puniyu_config::app::AppConfig;
use puniyu_context::PluginContext;
use puniyu_error::AnyError;
use puniyu_server::{Http, HttpMount, Server, ServerOptions};
use salvo::{Depot, FlowCtrl, Handler, Request, Response};
use semver::Version;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Debug, Default, Clone, Copy)]
pub struct Plugin;

struct Inner {
	server: Server,
}

struct AccessLogInner {
	mount: Mutex<Option<HttpMount>>,
}

#[async_trait]
impl puniyu_plugin_core::Plugin for Plugin {
	fn name(&self) -> &str {
		pkg_name!()
	}

	fn version(&self) -> Version {
		pkg_version!()
	}

	async fn on_start(&self, ctx: &PluginContext) -> AnyError {
		let config = AppConfig::get().server();
		let server = Server::new(ServerOptions {
			host: config.host(),
			port: config.port(),
			shutdown_timeout: Duration::from_secs(10),
		});
		let http = server.http();
		server.start().await?;

		if let Err(error) = ctx.provide(http.clone()) {
			server.stop().await?;
			return Err(Box::new(error));
		}
		let inner = Arc::new(Inner { server });
		if let Err(error) = ctx.provide(Arc::clone(&inner)) {
			ctx.remove::<Http>();
			inner.server.stop().await?;
			return Err(Box::new(error));
		}

		let mut mount = http.hoop(AccessLog);
		mount.mount()?;
		if let Err(error) = ctx.provide(Arc::new(AccessLogInner { mount: Mutex::new(Some(mount)) }))
		{
			return Err(Box::new(error));
		}
		Ok(())
	}

	async fn on_unload(&self, ctx: &PluginContext) -> AnyError {
		if let Some(inner) = ctx.remove::<Arc<AccessLogInner>>() {
			let mount = inner
				.mount
				.lock()
				.map_err(|_| std::io::Error::other("http mount lock is poisoned"))?
				.take();
			if let Some(mut mount) = mount {
				mount.unmount();
			}
		}
		ctx.require::<Arc<Inner>>()?.server.drain().await?;
		Ok(())
	}

	async fn on_stop(&self, ctx: &PluginContext) -> AnyError {
		if let Some(inner) = ctx.remove::<Arc<Inner>>() {
			inner.server.stop().await?;
		}
		ctx.remove::<Http>();
		Ok(())
	}
}

struct AccessLog;

#[async_trait]
impl Handler for AccessLog {
	async fn handle(
		&self,
		req: &mut Request,
		_depot: &mut Depot,
		res: &mut Response,
		ctrl: &mut FlowCtrl,
	) {
		let start = Instant::now();
		ctrl.call_next(req, _depot, res).await;
		let status = res.status_code.map(|value| value.as_u16().to_string()).unwrap_or_default();
		let ip = parse_ip(req)
			.or_else(|| req.remote_addr().ip())
			.map(|value| value.to_string())
			.unwrap_or_default();
		log::info!(
			"{} | {} | {} | {}ms | {}",
			req.method(),
			req.uri().path(),
			status,
			start.elapsed().as_millis(),
			ip
		);
	}
}

fn parse_ip(req: &Request) -> Option<IpAddr> {
	client_ip::x_real_ip(req.headers()).ok()
}
