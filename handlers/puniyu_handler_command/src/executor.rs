use crate::cooldown;
use crate::invocation::CommandInvocation;
use crate::policy::{denied_message, permission};
use log::{debug, error, info};
use puniyu_command::CommandAction;
use puniyu_context::MessageContext;
use puniyu_cooldown::CooldownState;
use puniyu_logger::owo_colors::OwoColorize;
use std::collections::HashMap;
use std::time::Instant;

pub(crate) enum CommandOutcome {
	Handled,
	Continue,
}

pub(crate) async fn execute(invocation: CommandInvocation<'_, '_>) -> CommandOutcome {
	let current_permission = permission(invocation.message);
	let invoked_name = invocation.parsed.command().to_owned();
	let mut denied = None;
	let mut argument_error = None;
	let mut executed = false;
	let mut cooldown_checked = false;

	for command in invocation.candidates {
		let required = command.permission();
		if !current_permission.satisfies(required) {
			denied.get_or_insert(required);
			continue;
		}

		let definitions = command.args();
		let arguments = match invocation.parsed.parse_args(&definitions) {
			Ok(arguments) => arguments,
			Err(err) => {
				argument_error = Some(err);
				continue;
			}
		};
		let context = MessageContext::new(invocation.message, arguments);

		if !cooldown_checked {
			if matches!(
				cooldown::check(invocation.message, invocation.options.cd),
				CooldownState::CoolingDown { .. }
			) {
				return CommandOutcome::Handled;
			}
			cooldown_checked = true;
		}

		executed = true;
		let name = command.name();
		let started_at = Instant::now();
		info!("[{}] 开始执行", format!("command:{name}").yellow());
		let result = command.execute(&context).await;
		let elapsed = started_at.elapsed().as_millis();

		match result {
			Ok(CommandAction::Done) => {
				info!("[{}] 执行完毕, 耗时{}ms", format!("command:{name}").yellow(), elapsed);
				return CommandOutcome::Handled;
			}
			Ok(CommandAction::Next) => {
				info!("[{}] 执行完毕, 耗时{}ms", format!("command:{name}").yellow(), elapsed);
				debug!("[{}] 继续传播", format!("command:{name}").yellow());
			}
			Err(err) => {
				error!(
					"[{}] 执行失败, 耗时{}ms: {}",
					format!("command:{name}").yellow(),
					elapsed,
					err
				);
				return CommandOutcome::Handled;
			}
		}
	}

	if executed {
		return CommandOutcome::Continue;
	}

	let context = MessageContext::new(invocation.message, HashMap::new());
	if let Some(error) = argument_error {
		if let Err(reply_error) = context.reply(error.to_string(), None).await {
			error!("[{}] 回复失败: {reply_error}", format!("command:{invoked_name}").yellow());
		}
		return CommandOutcome::Handled;
	}
	if let Some(required) = denied {
		if let Err(reply_error) = context.reply(denied_message(required), None).await {
			error!("[{}] 回复失败: {reply_error}", format!("command:{invoked_name}").yellow());
		}
		return CommandOutcome::Handled;
	}

	CommandOutcome::Continue
}
