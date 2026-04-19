use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TaskMetaData {
	pub name: String,
	pub cron: String,
}
