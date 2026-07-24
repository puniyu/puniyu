use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, LazyLock, Mutex};
use std::time::Duration;

use async_trait::async_trait;
use notify_debouncer_full::notify::RecommendedWatcher;
use notify_debouncer_full::{
	DebounceEventResult, Debouncer, RecommendedCache, new_debouncer, notify,
};
use puniyu_api::{pkg_name, pkg_version};
use puniyu_config::{ConfigRegistry, Entry};
use puniyu_context::PluginContext;
use puniyu_error::AnyError;
use puniyu_service::Service;
use semver::Version;

static CONFIGS: LazyLock<Mutex<Vec<Entry>>> = LazyLock::new(|| Mutex::new(Vec::new()));
static DEBOUNCER: LazyLock<Mutex<Option<Debouncer<RecommendedWatcher, RecommendedCache>>>> =
	LazyLock::new(|| Mutex::new(None));

macro_rules! log_prefix {
	($level:ident, $($arg:tt)+) => {{
		use ::puniyu_logger::owo_colors::OwoColorize;
		let prefix = "Config".fg_rgb::<255, 193, 7>();
		::log::$level!("[{}] {}", prefix, format_args!($($arg)+))
	}};
}

macro_rules! info  { ($($arg:tt)+) => { log_prefix!(info,  $($arg)+) }; }
macro_rules! warn  { ($($arg:tt)+) => { log_prefix!(warn,  $($arg)+) }; }
macro_rules! error { ($($arg:tt)+) => { log_prefix!(error, $($arg)+) }; }

fn should_reload(event: &notify::Event) -> bool {
	matches!(
		event.kind,
		notify::EventKind::Modify(_) | notify::EventKind::Create(_) | notify::EventKind::Remove(_)
	)
}

fn reload_config(path: &std::path::Path, id: u64, registry: &ConfigRegistry) {
	let content = match std::fs::read_to_string(path) {
		Ok(c) => c,
		Err(e) => {
			warn!("failed to read {}: {e}", path.display());
			return;
		}
	};
	let value = match toml::from_str::<toml::Value>(&content) {
		Ok(v) => v,
		Err(e) => {
			warn!("failed to parse {}: {e}", path.display());
			return;
		}
	};
	registry.get_mut(id, |v| *v = Entry { path: path.to_path_buf(), value });
	info!("config reloaded: {}", path.display());
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

		let path_ids = {
			let configs = CONFIGS.lock().expect("poisoned lock");
			configs
				.iter()
				.map(|entry| {
					let id = registry.insert(entry.clone());
					(entry.path.clone(), id)
				})
				.collect::<HashMap<PathBuf, u64>>()
		};

		let registry = registry.clone();
		let path_ids = Arc::new(path_ids);

		let mut debouncer =
			new_debouncer(Duration::from_secs(1), None, move |res: DebounceEventResult| {
				let events = match res {
					Ok(events) => events,
					Err(e) => {
						error!("config watch error: {e:?}");
						return;
					}
				};
				for event in &events {
					if !should_reload(&event.event) {
						continue;
					}
					for path in &event.event.paths {
						if path.extension().is_some_and(|ext| ext == "toml")
							&& let Some(&id) = path_ids.get(path)
						{
							reload_config(path, id, &registry);
						}
					}
				}
			})
			.expect("failed to create config file watcher");

		debouncer
			.watch(puniyu_path::config_dir(), notify::RecursiveMode::NonRecursive)
			.expect("failed to start watching config directory");

		DEBOUNCER.lock().expect("poisoned lock").replace(debouncer);

		Ok(())
	}
}
