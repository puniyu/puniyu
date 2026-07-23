use async_trait::async_trait;
use bytes::Bytes;
use puniyu_api::{pkg_name, pkg_version};
use puniyu_context::PluginContext;
use puniyu_error::AnyError;
use puniyu_server::Http;
use puniyu_service::Service;
use salvo::{Depot, FlowCtrl, Handler, Request, Response, Router, routing::PathFilter};
use semver::Version;
use std::sync::OnceLock;

#[derive(Debug, Clone, Default)]
pub struct Logo(Bytes);

impl From<Logo> for Bytes {
	fn from(logo: Logo) -> Self {
		logo.0
	}
}

static DATA: OnceLock<Logo> = OnceLock::new();

pub struct Plugin;

impl Plugin {
	pub fn with_logo(data: impl Into<Bytes>) -> Self {
		DATA.set(Logo(data.into())).expect("logo data already set");
		Self
	}
}

#[async_trait]
impl puniyu_plugin_core::Plugin for Plugin {
	fn name(&self) -> &str {
		pkg_name!()
	}

	fn version(&self) -> Version {
		pkg_version!()
	}

	fn using(&self) -> Vec<&str> {
		vec![puniyu_service_server::Service {}.name()]
	}

	async fn on_load(&self, ctx: &PluginContext) -> AnyError {
		let data = DATA.get().cloned().unwrap_or_default();
		tokio::fs::write(puniyu_path::assets_dir().join("logo.png"), data.0.clone()).await?;
		let mut mount = ctx.require::<Http>()?.router(move || {
			Router::new().filter(PathFilter::new("logo")).get(LogoHandler(data.clone()))
		});
		mount.mount()?;
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
