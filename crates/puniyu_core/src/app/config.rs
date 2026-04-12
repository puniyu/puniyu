use puniyu_common::{merge_config, read_config};
use puniyu_error::Result;
use puniyu_config::{Config, ConfigRegistry};
use puniyu_path::config_dir;
use std::{io::Error, io::ErrorKind, path::Path, sync::Arc};
use tokio::fs::create_dir_all;
use toml::Value;

pub async fn init_config(name: &str, configs: Vec<Arc<dyn Config>>) -> Result {
	if configs.is_empty() {
		return Ok(());
	}

	for config in configs {
		let mut config = config.config();
		let path = config_dir().join(name).join(&config.name);
		if let Some(parent) = path.parent()
			&& !parent.exists()
		{
			create_dir_all(parent).await.map_err(|e| {
				Error::other(format!("Failed to create parent config dir for {}: {}", name, e))
			})?;
		}
		let existing = read_config_value(&path, &config.name, name)?;
		merge_config(config_dir().join(name), &config.name, &config.value, &existing).map_err(|e| {
			Error::other(format!("Failed to merge config {} for {}: {}", config.name, name, e))
		})?;
		config.path = path.clone();
		config.value = read_config_value(&path, &config.name, name)?;
		ConfigRegistry::register(config).map_err(|e| {
			Error::other(format!("Failed to register config for {}: {:?}", name, e))
		})?;
	}

	Ok(())
}

fn read_config_value(path: &Path, config_name: &str, owner_name: &str) -> Result<Value> {
	let dir = path.parent().ok_or_else(|| {
		Error::other(format!(
			"Failed to get parent config dir for {}: {}",
			owner_name, config_name
		))
	})?;
	let stem = path.file_stem().and_then(|stem| stem.to_str()).ok_or_else(|| {
		Error::other(format!(
			"Failed to get config file stem for {}: {}",
			owner_name, config_name
		))
	})?;
	match read_config::<Value>(dir, stem) {
		Ok(value) => Ok(value),
		Err(puniyu_error::config::Error::Io(e)) if e.kind() == ErrorKind::NotFound => {
			Ok(Value::Table(Default::default()))
		}
		Err(e) => Err(Error::other(format!(
			"Failed to read config {} for {}: {}",
			config_name, owner_name, e
		))
		.into()),
	}
}
