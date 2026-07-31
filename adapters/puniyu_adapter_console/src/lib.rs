mod bot;

use puniyu_api::{pkg_name, pkg_version};

pub struct Plugin;


#[async_trait::async_trait]
impl puniyu_plugin::Plugin for Plugin {
	fn name(&self) -> &str {
		pkg_name!()
	}

	fn version(&self) -> semver::Version {
		pkg_version!()
	}

	async fn on_start(&self, ctx: &puniyu_context::SubContext) -> puniyu_error::AnyError {
		let bot = bot::Bot::new(ctx.path().assets_dir());
		ctx.bot().insert(bot);
		Ok(())
	}

	async fn on_stop(&self, ctx: &puniyu_context::SubContext) -> puniyu_error::AnyError {
		ctx.bot().remove(bot::BOT_ID);
		Ok(())
	}
}