use async_trait::async_trait;
use bytes::Bytes;
use puniyu_api::{pkg_name, pkg_version};
use puniyu_context::PluginContext;
use puniyu_error::AnyError;
use puniyu_server::{Http, HttpMount};
use salvo::{Depot, FlowCtrl, Handler, Request, Response, Router};
use semver::Version;
use std::sync::{Arc, Mutex, OnceLock};

#[derive(Clone)]
pub struct Logo(pub Bytes);

impl From<Logo> for Bytes {
	fn from(logo: Logo) -> Self {
		logo.0
	}
}

static DATA: OnceLock<Bytes> = OnceLock::new();

pub struct Plugin;

impl Plugin {
	pub fn with_logo(data: impl Into<Bytes>) -> Self {
		DATA.set(data.into()).expect("logo data already set");
		Self
	}
}

struct Inner {
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
		let data = DATA.get().cloned().unwrap_or_default();
		tokio::fs::write(puniyu_path::assets_dir().join("logo.png"), data.clone()).await?;
		ctx.provide(Logo(data))?;
		Ok(())
	}

	async fn on_load(&self, ctx: &PluginContext) -> AnyError {
		let data = ctx.require::<Logo>()?;
		let mut mount = ctx
			.require::<Http>()?
			.router(move || Router::with_path("logo").get(LogoHandler(data.clone())));
		mount.mount()?;
		ctx.provide(Arc::new(Inner { mount: Mutex::new(Some(mount)) }))?;
		Ok(())
	}

	async fn on_unload(&self, ctx: &PluginContext) -> AnyError {
		if let Some(inner) = ctx.remove::<Arc<Inner>>() {
			let mount = inner
				.mount
				.lock()
				.map_err(|_| std::io::Error::other("http mount lock is poisoned"))?
				.take();
			if let Some(mut mount) = mount {
				mount.unmount();
			}
		}
		Ok(())
	}
}

struct LogoHandler(Logo);

#[async_trait]
impl Handler for LogoHandler {
	async fn handle(
		&self,
		_req: &mut Request,
		_depot: &mut Depot,
		res: &mut Response,
		ctrl: &mut FlowCtrl,
	) {
		let bytes: Bytes = self.0.clone().into();
		if bytes.is_empty() {
			res.status_code(salvo::http::StatusCode::NOT_FOUND);
			return;
		}
		res.add_header("content-type", salvo::http::mime::IMAGE_PNG.as_ref(), true).ok();
		res.write_body(bytes).ok();
		ctrl.skip_rest();
	}
}
