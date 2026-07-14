mod bot;
#[doc(inline)]
pub use bot::BotContext;
mod event;
#[doc(inline)]
pub use event::*;

use puniyu_command_types::ArgValue;
use smol_str::SmolStr;
use std::collections::HashMap;

/// 命令参数映射类型
pub type EventArg = HashMap<SmolStr, Vec<ArgValue>>;
