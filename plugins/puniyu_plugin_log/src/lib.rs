use async_trait::async_trait;
use puniyu_api::{pkg_name, pkg_version};
use puniyu_context::PluginContext;
use puniyu_error::AnyError;
use semver::Version;
#[cfg(feature = "event_log")]
use {
	log::info,
	puniyu_core::event::EventBase,
	puniyu_element::receive::Elements,
	puniyu_event::{EventType, message::MessageEvent},
	puniyu_logger::owo_colors::OwoColorize,
	puniyu_middleware::{Middleware, MiddlewareContext},
	puniyu_plugin_event::EventEmitter,
	std::sync::Arc,
	std::time::Instant,
};

#[cfg(feature = "access_log")]
use {
	puniyu_server::{Http, HttpMount},
	salvo::{Depot, FlowCtrl, Handler, Request, Response},
	std::net::IpAddr,
	std::sync::Mutex,
};

pub struct Plugin;

#[async_trait]
impl puniyu_plugin_core::Plugin for Plugin {
	fn name(&self) -> &str {
		pkg_name!()
	}

	fn priority(&self) -> u32 {
		1
	}

	fn version(&self) -> Version {
		pkg_version!()
	}

	async fn on_start(&self, _ctx: &PluginContext) -> AnyError {
		log_init();
		Ok(())
	}

	async fn on_load(&self, ctx: &PluginContext) -> AnyError {
		#[cfg(feature = "event_log")]
		{
			let emitter = ctx.require::<EventEmitter>()?;
			let middleware: Arc<dyn Middleware> = Arc::new(EventLog);
			emitter.on(EventType::Message, Arc::clone(&middleware))?;
			if let Err(error) =
				ctx.provide(Arc::new(EventLogInner { middleware: Arc::clone(&middleware) }))
			{
				emitter.off(EventType::Message, Arc::clone(&middleware));
				return Err(Box::new(error));
			}
		}

		#[cfg(feature = "access_log")]
		{
			let mut mount = ctx.require::<Http>()?.hoop(AccessLog);
			mount.mount()?;
			ctx.provide(Arc::new(AccessLogInner { mount: Mutex::new(Some(mount)) }))?;
		}
		Ok(())
	}

	async fn on_unload(&self, ctx: &PluginContext) -> AnyError {
		#[cfg(feature = "event_log")]
		{
			let emitter = ctx.require::<EventEmitter>()?;
			if let Some(inner) = ctx.remove::<Arc<EventLogInner>>() {
				emitter.off(EventType::Message, Arc::clone(&inner.middleware));
			}
		}

		#[cfg(feature = "access_log")]
		{
			if let Some(inner) = ctx.remove::<Arc<AccessLogInner>>() {
				let mount = inner
					.mount
					.lock()
					.map_err(|_| std::io::Error::other("http mount lock is poisoned"))?
					.take();
				if let Some(mut mount) = mount {
					mount.unmount();
				}
			}
		}
		Ok(())
	}
}

fn log_init() {
	use puniyu_config::app::AppConfig;
	use puniyu_logger::{LogLevel, LoggerOptions};
	use puniyu_path::log_dir;
	use std::{env, str::FromStr};

	let config = AppConfig::get().logger();
	let log_level = env::var("LOGGER_LEVEL").unwrap_or(config.level().to_string());
	let log_path = log_dir().to_string_lossy().to_string();
	let log_retention_days = config.retention_days();
	let is_file_logging = config.enable_file();
	let options = LoggerOptions::default()
		.with_prefix(puniyu_app::App::name())
		.with_level(LogLevel::from_str(log_level.as_str()).unwrap_or(LogLevel::Info))
		.with_file_logging(is_file_logging)
		.with_log_directory(log_path)
		.with_retention_days(log_retention_days);
	puniyu_logger::init(Some(options));
}

#[cfg(feature = "event_log")]
struct EventLogInner {
	middleware: Arc<dyn Middleware>,
}

#[cfg(feature = "event_log")]
#[derive(Debug, Default, Clone, Copy)]
struct EventLog;

#[cfg(feature = "event_log")]
#[async_trait]
impl Middleware for EventLog {
	fn name(&self) -> &'static str {
		"event_log"
	}

	fn priority(&self) -> u32 {
		100
	}

	async fn handle(&self, mut ctx: MiddlewareContext<'_>) {
		let Some(message) = ctx.as_message() else {
			ctx.next().await;
			return;
		};

		let event_id = message.event_id().to_owned();
		info!("{}", format_message(message));
		let started_at = Instant::now();
		ctx.next().await;
		let elapsed = started_at.elapsed().as_millis();
		info!("[{}:{}] 处理完成, 耗时{}ms", "Event".yellow(), event_id.green(), elapsed);
	}
}

#[cfg(feature = "event_log")]
fn format_message(event: &MessageEvent) -> String {
	let raw_message = event.elements().iter().map(format_element).collect::<Vec<_>>().join("");

	if let Some(event) = event.as_group() {
		return format!(
			"[{}:{}][{}:{}-{}]: {}",
			"Bot".yellow(),
			event.self_id().green(),
			"GroupMessage".yellow(),
			event.group_id().green(),
			event.user_id().green(),
			raw_message
		);
	}
	if let Some(event) = event.as_group_temp() {
		return format!(
			"[{}:{}][{}:{}-{}]: {}",
			"Bot".yellow(),
			event.self_id().green(),
			"GroupTempMessage".yellow(),
			event.group_id().green(),
			event.user_id().green(),
			raw_message
		);
	}
	if let Some(event) = event.as_guild() {
		return format!(
			"[{}:{}][{}:{}-{}]: {}",
			"Bot".yellow(),
			event.self_id().green(),
			"GuildMessage".yellow(),
			event.guild_id().green(),
			event.user_id().green(),
			raw_message
		);
	}

	format!(
		"[{}:{}][{}:{}]: {}",
		"Bot".yellow(),
		event.self_id().green(),
		"FriendMessage".yellow(),
		event.user_id().green(),
		raw_message
	)
}

#[cfg(feature = "event_log")]
fn format_element(element: &Elements) -> String {
	match element {
		Elements::Text(value) => format!("text:{}", value.text),
		Elements::At(value) => format!("at:{}", value.target_id),
		Elements::Reply(value) => format!("reply:{}", value.message_id),
		Elements::Face(value) => format!("face:{}", value.id),
		Elements::Image(value) => {
			format!("image:{}", value.summary.as_deref().unwrap_or(value.file_name.as_str()))
		}
		Elements::File(value) => format!("file:{}", value.file_name),
		Elements::Video(value) => format!("video:{}", value.file_name),
		Elements::Record(value) => format!("record:{}", value.file_name),
		Elements::Json(value) => format!("json:{}", value.data),
		Elements::Xml(value) => format!("xml:{}", value.data),
	}
}

#[cfg(feature = "access_log")]
struct AccessLogInner {
	mount: Mutex<Option<HttpMount>>,
}

#[cfg(feature = "access_log")]
struct AccessLog;

#[cfg(feature = "access_log")]
#[async_trait]
impl Handler for AccessLog {
	async fn handle(
		&self,
		req: &mut Request,
		_depot: &mut Depot,
		res: &mut Response,
		ctrl: &mut FlowCtrl,
	) {
		let start = Instant::now();
		ctrl.call_next(req, _depot, res).await;
		let status = res.status_code.map(|value| value.as_u16().to_string()).unwrap_or_default();
		let ip = parse_ip(req)
			.or_else(|| req.remote_addr().ip())
			.map(|value| value.to_string())
			.unwrap_or_default();
		log::info!(
			"{} | {} | {} | {}ms | {}",
			req.method(),
			req.uri().path(),
			status,
			start.elapsed().as_millis(),
			ip
		);
	}
}

#[cfg(feature = "access_log")]
fn parse_ip(req: &Request) -> Option<IpAddr> {
	client_ip::x_real_ip(req.headers()).ok()
}
