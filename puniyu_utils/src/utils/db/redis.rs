use crate::config::Config;
use deadpool_redis::{Config as DpConfig, Pool, Runtime};
use redis::cmd;
use std::process;
use std::sync::OnceLock;

pub static REDIS_CLIENT: OnceLock<Pool> = OnceLock::new();

pub async fn init_redis() {
    let redis_config = Config::redis();
    let auth_part = if redis_config.password.is_empty() {
        "".to_string()
    } else {
        format!(":{}@", redis_config.password)
    };

    let db_part = if redis_config.db > 0 {
        format!("/{}", redis_config.db)
    } else {
        "".to_string()
    };

    let url = format!(
        "redis://{}{}:{}{}",
        auth_part, redis_config.host, redis_config.port, db_part
    );

    let dp_cfg = DpConfig::from_url(url);

    let pool: Pool = match dp_cfg.create_pool(Some(Runtime::Tokio1)) {
        Ok(p) => p,
        Err(e) => {
            log::error!("[Redis] Redis 连接失败：{:?}", e);
            process::exit(1);
        }
    };
    match pool.get().await {
        Ok(mut conn) => {
            if let Err(e) = redis::cmd("PING").query_async::<String>(&mut conn).await {
                log::error!("[Redis] Redis 连接失败：{:?}", e);
                process::exit(1);
            }
        }
        Err(e) => {
            log::error!("[Redis] Redis 连接失败：{:?}", e);
            process::exit(1);
        }
    }

    if REDIS_CLIENT.set(pool).is_err() {
        log::error!("[Redis] Redis 连接池初始化失败：已被初始化");
    } else {
        log::debug!("[Redis] Redis 连接成功");
    }
}

pub struct Redis {}

impl Redis {
    /// 获取键对应的值
    ///
    /// # 参数
    ///
    /// * `key` - 键
    ///
    /// # 返回值
    ///
    /// * `Option<T>` - 键对应的值
    ///
    /// # 泛型参数
    ///
    /// * `T` - 键对应的值的类型
    pub async fn get<T>(key: &str) -> Option<T>
    where
        T: redis::FromRedisValue,
    {
        let pool = REDIS_CLIENT.get().unwrap();
        let mut conn = pool.get().await.unwrap();
        let res: Option<T> = cmd("GET").arg(key).query_async(&mut conn).await.unwrap();
        res
    }

    /// 设置键值对
    ///
    /// # 参数
    ///
    /// * `key` - 键
    /// * `value` - 值
    ///
    /// # 返回值
    ///
    /// * `bool` - 是否设置成功
    ///
    /// # 泛型参数
    ///
    /// * `T` - 值的类型
    ///
    /// # 示例
    ///
    /// ```
    /// use puniyu_utils::utils::db::redis::Redis;
    ///
    /// async fn main() {
    ///     Redis::set("key", "value").await;
    /// }
    /// ```
    pub async fn set<T>(key: &str, value: T) -> bool
    where
        T: redis::ToRedisArgs,
    {
        let pool = REDIS_CLIENT.get().unwrap();
        let mut conn = pool.get().await.unwrap();
        let res: bool = cmd("SET")
            .arg(key)
            .arg(value)
            .query_async(&mut conn)
            .await
            .unwrap();
        res
    }

    /// 获取所有键
    pub async fn all() -> Vec<String> {
        let pool = REDIS_CLIENT.get().unwrap();
        let mut conn = pool.get().await.unwrap();
        let res: Vec<String> = cmd("KEYS").arg("*").query_async(&mut conn).await.unwrap();
        res
    }
}
