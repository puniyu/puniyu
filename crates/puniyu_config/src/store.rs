use crate::StoredEntry;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{LazyLock, RwLock};

static STORE: LazyLock<RwLock<HashMap<u64, (u64, String, std::path::PathBuf, toml::Value)>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));
static COUNTER: AtomicU64 = AtomicU64::new(0);

pub struct ConfigStore;

impl ConfigStore {
    pub fn register<T>(
        scope_id: u64,
        name: &str,
        path: impl AsRef<Path>,
        config: T,
    ) -> u64
    where
        T: Serialize + for<'de> Deserialize<'de> + Clone + Send + Sync + 'static,
    {
        let value = match toml::Value::try_from(&config) {
            Ok(v) => v,
            Err(e) => {
                log::error!("failed to serialize config '{}': {}", name, e);
                return 0;
            }
        };

        let mut map = STORE.write().expect("poisoned lock");
        if map.values().any(|(_, n, _, _)| n == name) {
            log::warn!("config '{}' already registered, skipping", name);
            return map
                .iter()
                .find(|(_, (_, n, _, _))| n == name)
                .map(|(k, _)| *k)
                .unwrap_or(0);
        }

        let index = COUNTER.fetch_add(1, Ordering::Relaxed);
        map.insert(
            index,
            (
                scope_id,
                name.to_string(),
                path.as_ref().to_path_buf(),
                value,
            ),
        );
        index
    }

    pub fn get<T>(path: impl AsRef<Path>) -> Option<T>
    where
        T: Serialize + for<'de> Deserialize<'de> + Clone + Send + Sync + 'static,
    {
        let map = STORE.read().expect("poisoned lock");
        map.values()
            .find(|(_, _, p, _)| p == path.as_ref())
            .and_then(|(_, _, _, v)| {
                toml::to_string(v)
                    .ok()
                    .and_then(|s| toml::from_str(&s).ok())
            })
    }

    pub fn get_by_name<T>(name: &str) -> Option<T>
    where
        T: Serialize + for<'de> Deserialize<'de> + Clone + Send + Sync + 'static,
    {
        let map = STORE.read().expect("poisoned lock");
        map.values()
            .find(|(_, n, _, _)| n == name)
            .and_then(|(_, _, _, v)| {
                toml::to_string(v)
                    .ok()
                    .and_then(|s| toml::from_str(&s).ok())
            })
    }

    pub fn update<T>(path: impl AsRef<Path>, value: T)
    where
        T: Serialize + for<'de> Deserialize<'de> + Clone + Send + Sync + 'static,
    {
        let toml_val = match toml::Value::try_from(&value) {
            Ok(v) => v,
            Err(e) => {
                log::error!("failed to serialize config update: {}", e);
                return;
            }
        };

        let mut map = STORE.write().expect("poisoned lock");
        if let Some((_, _, _, v)) = map.values_mut().find(|(_, _, p, _)| p == path.as_ref()) {
            *v = toml_val;
        }
    }

    pub fn all<T>() -> Vec<StoredEntry<T>>
    where
        T: Serialize + for<'de> Deserialize<'de> + Clone + Send + Sync + 'static,
    {
        let map = STORE.read().expect("poisoned lock");
        map.values()
            .filter_map(|(_, n, p, v)| {
                toml::to_string(v)
                    .ok()
                    .and_then(|s| toml::from_str(&s).ok())
                    .map(|value| StoredEntry {
                        name: n.clone().into(),
                        path: p.clone(),
                        value,
                    })
            })
            .collect()
    }

    pub fn remove_by_scope(scope_id: u64) {
        STORE
            .write()
            .expect("poisoned lock")
            .retain(|_, (sid, _, _, _)| *sid != scope_id);
    }
}
