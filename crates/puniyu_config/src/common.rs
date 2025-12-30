use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ReactiveMode {
	/// 所有人
	All = 0,
	/// 仅艾特Bot
	AtBot = 1,
	/// 仅别名
	Alias = 2,
	/// 别名或艾特
	AtOrAlias = 3,
	/// 仅主人
	Master = 4,
}
