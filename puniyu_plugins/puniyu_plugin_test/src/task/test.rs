use puniyu_core::plugin::prelude::*;

#[task(cron = "* * * * * *")]
pub async fn test() {}
