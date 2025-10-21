use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

#[derive(Debug, Clone, PartialEq, EnumString, Display, IntoStaticStr, Deserialize, Serialize)]
pub enum AvatarSize {
	#[strum(serialize = "small")]
	Small,
	#[strum(serialize = "medium")]
	Medium,
	#[strum(serialize = "large")]
	Large,
}
