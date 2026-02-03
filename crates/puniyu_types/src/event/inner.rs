use crate::bot::Bot;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub(crate) fn serialize_bot<S>(bot_info: &Arc<Bot>, serializer: S) -> Result<S::Ok, S::Error>
where
	S: serde::Serializer,
{
	bot_info.serialize(serializer)
}

pub(crate) fn deserialize_bot<'de, D>(deserializer: D) -> Result<Arc<Bot>, D::Error>
where
	D: serde::Deserializer<'de>,
{
	let bot_info = Bot::deserialize(deserializer)?;
	Ok(Arc::new(bot_info))
}
