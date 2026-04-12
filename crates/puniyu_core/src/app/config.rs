use puniyu_common::{merge_config, read_config};
use puniyu_config::{Config, ConfigRegistry};
use puniyu_path::config_dir;
use std::{io::ErrorKind, path::Path, sync::Arc};
use tokio::fs::create_dir_all;
use toml::Value;

pub async fn init_config(name: &str, configs: Vec<Arc<dyn Config>>) {
	if configs.is_empty() {
		return;
	}

	for config in configs {
		let config = config.config();
		let path = config_dir().join(name).join(&config.name);
		if let Some(parent) = path.parent()
			&& !parent.exists()
		{
			create_dir_all(parent).await.unwrap_or_else(|e| {
				panic!(
					"Failed to create parent config dir for {}: {}",
					name, e
				)
			});
		}
		let existing = read_config_value(&path, &config.name, name);
		merge_config(config_dir().join(name), &config.name, &config.value, &existing)
			.unwrap_or_else(|e| panic!("Failed to merge config {} for {}: {}", config.name, name, e));
		let mut config = config;
		config.value = read_config_value(&path, &config.name, name);
		ConfigRegistry::register(config)
			.unwrap_or_else(|e| panic!("Failed to register config for {}: {:?}", name, e));
	}
}

fn read_config_value(path: &Path, config_name: &str, owner_name: &str) -> Value {
	let dir = path.parent().unwrap_or_else(|| {
		panic!(
			"Failed to get parent config dir for {}: {}",
			owner_name, config_name
		)
	});
	let stem = path.file_stem().and_then(|stem| stem.to_str()).unwrap_or_else(|| {
		panic!(
			"Failed to get config file stem for {}: {}",
			owner_name, config_name
		)
	});
	match read_config::<Value>(dir, stem) {
		Ok(value) => value,
		Err(puniyu_error::config::Error::Io(e)) if e.kind() == ErrorKind::NotFound => {
			Value::Table(Default::default())
		}
		Err(e) => panic!("Failed to read config {} for {}: {}", config_name, owner_name, e),
	}
}
