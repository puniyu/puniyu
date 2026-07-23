use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{LazyLock, Mutex};
use std::thread;
use std::time::Duration;

use async_trait::async_trait;
use notify_debouncer_full::{DebounceEventResult, new_debouncer, notify};
use puniyu_api::{pkg_name, pkg_version};
use puniyu_config::{ConfigRegistry, Entry};
use puniyu_context::PluginContext;
use puniyu_error::AnyError;
use puniyu_service::Service;
use semver::Version;

static CONFIGS: LazyLock<Mutex<Vec<Entry>>> = LazyLock::new(|| Mutex::new(Vec::new()));

macro_rules! info {
	($($arg:tt)+) => {{
		use ::puniyu_logger::owo_colors::OwoColorize;
        let prefix = "Config".fg_rgb::<255, 193, 7>();
		::log::info!("[{}] {}", prefix, format_args!($($arg)+))
	}};
}

macro_rules! warn {
	($($arg:tt)+) => {{
		use ::puniyu_logger::owo_colors::OwoColorize;
        let prefix = "Config".fg_rgb::<255, 193, 7>();
		::log::warn!("[{}] {}", prefix, format_args!($($arg)+))
	}};
}

macro_rules! error {
	($($arg:tt)+) => {{
		use ::puniyu_logger::owo_colors::OwoColorize;
        let prefix = "Config".fg_rgb::<255, 193, 7>();
		::log::error!("[{}] {}", prefix, format_args!($($arg)+))
	}};
}
pub struct Plugin;

impl Plugin {
	pub fn with_config(self, entry: impl Into<Entry>) -> Self {
		CONFIGS.lock().expect("poisoned lock").push(entry.into());
		self
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
		vec![puniyu_service_config::Service {}.name()]
	}

	async fn on_load(&self, ctx: &PluginContext) -> AnyError {
		let registry = ctx.require::<ConfigRegistry>()?;

		let configs = CONFIGS.lock().expect("poisoned lock");
		let mut path_ids: HashMap<PathBuf, u64> = HashMap::new();
		for entry in configs.iter() {
			let id = registry.insert(entry.clone());
			path_ids.insert(entry.path.clone(), id);
		}
		drop(configs);

		let registry = registry.clone();
		let path_ids = std::sync::Arc::new(path_ids);

		thread::spawn(move || {
			let mut debouncer =
				new_debouncer(Duration::from_secs(1), None, move |res: DebounceEventResult| {
					match res {
						Ok(events) => {
							for event in events.iter() {
								if !matches!(
									event.event.kind,
									notify::EventKind::Modify(_)
										| notify::EventKind::Create(_) | notify::EventKind::Remove(_)
								) {
									continue;
								}
								for path in &event.event.paths {
									if !path.extension().is_some_and(|ext| ext == "toml") {
										continue;
									}
									let Some(&id) = path_ids.get(path) else {
										continue;
									};
									match std::fs::read_to_string(path) {
										Ok(content) => {
											match toml::from_str::<toml::Value>(&content) {
												Ok(value) => {
													registry.get_mut(id, |v| {
														*v = Entry {
															path: path.clone(),
															value,
														}
													});
													info!(
														"config reloaded: {}",
														path.display()
													);
												}
												Err(e) => warn!(
													"failed to parse {}: {e}",
													path.display()
												),
											}
										}
										Err(e) => {
											warn!("failed to read {}: {e}", path.display())
										}
									}
								}
							}
						}
						Err(e) => error!("config watch error: {e:?}"),
					}
				})
				.expect("failed to create config file watcher");

			let config_dir = puniyu_path::config_dir();
			debouncer
				.watch(&config_dir, notify::RecursiveMode::NonRecursive)
				.expect("failed to start watching config directory");

			thread::park();
		});

		Ok(())
	}
}
