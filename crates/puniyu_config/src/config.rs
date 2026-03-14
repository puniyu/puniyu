use crate::app::APP_CONFIG;
use crate::bot::BOT_CONFIG;
use crate::friend::FRIEND_CONFIG;
use crate::group::GROUP_CONFIG;
use puniyu_common::read_config;
use puniyu_error::config::Error;
use puniyu_logger::{debug, error, info};
use puniyu_path::CONFIG_DIR;
use serde::Deserialize;
use std::thread;
use std::time::Duration;
pub fn reload_config<T>(name: &str, config: &mut T) -> Result<(), Error>
where
	T: Default + for<'de> Deserialize<'de>,
{
	*config = read_config(CONFIG_DIR.as_path(), name).unwrap_or_else(|_| T::default());
	Ok(())
}

#[cfg(feature = "watcher")]
pub fn start_config_watcher() {
	use notify_debouncer_full::{DebounceEventResult, new_debouncer, notify};
	thread::spawn(|| {
		debug!("[Config] 配置文件监听器已启动");

		let mut debouncer =
			new_debouncer(Duration::from_secs(2), None, |res: DebounceEventResult| match res {
				Ok(events) => {
					for event in events.iter() {
						if !matches!(
							event.event.kind,
							notify::EventKind::Modify(_)
								| notify::EventKind::Create(_)
								| notify::EventKind::Remove(_)
						) {
							continue;
						}

						for path in event.event.paths.iter() {
							info!("[Config] 文件变更: {}", path.display());

							if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
								match file_name {
									"app.toml" => {
										reload_config(
											"app",
											&mut *APP_CONFIG
												.write()
												.expect("Failed to acquire app config write lock"),
										)
										.expect("Failed to reload app config");
										info!("[Config] App 配置已重载");
									}
									"bot.toml" => {
										reload_config(
											"bot",
											&mut *BOT_CONFIG
												.write()
												.expect("Failed to acquire bot config write lock"),
										)
										.expect("Failed to reload bot config");
										info!("[Config] Bot 配置已重载");
									}
									"group.toml" => {
										reload_config(
											"group",
											&mut *GROUP_CONFIG.write().expect(
												"Failed to acquire group config write lock",
											),
										)
										.expect("Failed to reload group config");
										info!("[Config] Group 配置已重载");
									}
									"friend.toml" => {
										reload_config(
											"friend",
											&mut *FRIEND_CONFIG.write().expect(
												"Failed to acquire friend config write lock",
											),
										)
										.expect("Failed to reload friend config");
										info!("[Config] Friend 配置已重载");
									}
									_ => {}
								}
								#[cfg(feature = "registry")]
								{
									if crate::ConfigRegistry::all().iter().any(|c| c.path == *path)
										&& let Some(name) =
											path.file_stem().and_then(|s| s.to_str())
										&& let Some(dir) = path.parent()
										&& let Ok(value) = read_config::<toml::Value>(dir, name)
									{
										crate::ConfigRegistry::update(
											path.as_path(),
											value,
										)
										.expect("Failed to update config");
										debug!("[Config] 配置已更新, {name}, {path}", name=name, path=path.display())
									}
								}
							}
						}
					}
				}
				Err(e) => error!("[Config] 监听错误: {:?}", e),
			})
			.expect("Failed to create config file watcher");

		debouncer
			.watch(CONFIG_DIR.as_path(), notify::RecursiveMode::Recursive)
			.expect("Failed to start watching config directory");

		thread::park();
	});
}
