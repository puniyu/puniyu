use crate::{ActionConetxt, BotContext, PathConetxt, ServiceContext};
use std::sync::Arc;

pub struct SubContext {
	pub(crate) bot: Arc<BotContext>,
	pub(crate) service: Arc<ServiceContext>,
	pub(crate) action: Arc<ActionConetxt>,
	pub(crate) path: Arc<PathConetxt>,
}

impl SubContext {
	pub fn bot(&self) -> &BotContext {
		&self.bot
	}
	pub fn service(&self) -> &ServiceContext {
		&self.service
	}
	pub fn action(&self) -> &ActionConetxt {
		&self.action
	}
	pub fn path(&self) -> &PathConetxt {
		&self.path
	}
}
