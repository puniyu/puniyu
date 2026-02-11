use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

#[derive(Debug, Default, Clone, PartialEq, EnumString, Display, IntoStaticStr, Deserialize, Serialize)]
pub enum Permission {
    /// 所有人
    #[strum(serialize = "all")]
    #[default]
    All,
    /// 仅主人
    #[strum(serialize = "master")]
    Master,
}