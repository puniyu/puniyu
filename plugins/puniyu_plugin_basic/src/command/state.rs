use puniyu_common::system::SystemInfo;
use puniyu_core::APP_NAME;
use puniyu_core::VERSION;
use puniyu_plugin::prelude::*;
#[command(name = "state")]
async fn state(ctx: &MessageContext) -> HandlerResult<HandlerAction> {
	let status = SystemInfo::new();
	let days = status.run_time / 86400;
	let hours = (status.run_time % 86400) / 3600;
	let minutes = (status.run_time % 3600) / 60;
	let seconds = status.run_time % 60;

	let info_text = format!(
		"------应用状态------\n\
		框架名称: {}\n\
		当前版本: v{}\n\
		内存占用：{:.2}MB\n\
		运行时间：{}天{}小时{}分钟{}秒",
		APP_NAME.get().unwrap(),
		VERSION,
		status.used_memory,
		days,
		hours,
		minutes,
		seconds
	);
	ctx.reply(message!(segment!(text, info_text))).await?;
	Ok(HandlerAction::Done)
}
