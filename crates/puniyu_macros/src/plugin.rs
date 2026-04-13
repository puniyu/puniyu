mod command;
pub use command::command;
mod task;
pub use task::task;
mod config;
pub use config::config;
mod hook;
use crate::{
	PluginArg,
	common::{validate_async, validate_return_type},
};
pub use hook::hook;
use zyn::{ToTokens, zyn};

pub fn plugin(item: zyn::syn::ItemFn, cfg: PluginArg) -> zyn::TokenStream {
	if let Err(err) = validate_async(&item.sig) {
		return err.to_compile_error();
	}
	if !item.block.stmts.is_empty()
		&& let Err(err) = validate_return_type(&item.sig, "puniyu_plugin::Result")
	{
		return err.to_compile_error();
	}

	let plugin_desc = match &cfg.desc {
		Some(desc) => zyn! { Some({{ desc}}) },
		None => zyn! { None },
	};
	let plugin_prefix = match &cfg.prefix {
		Some(prefix) => zyn! { Some({{ prefix }}) },
		None => zyn! { None },
	};
	let plugin_name = zyn! { env!("CARGO_PKG_NAME") };
	let version_major = zyn! { env!("CARGO_PKG_VERSION_MAJOR").parse::<u64>().unwrap() };
	let version_minor = zyn! { env!("CARGO_PKG_VERSION_MINOR").parse::<u64>().unwrap() };
	let version_patch = zyn! { env!("CARGO_PKG_VERSION_PATCH").parse::<u64>().unwrap() };
	let fn_name = &item.sig.ident;
	let plugin_author = zyn! {
		{
			let authors = env!("CARGO_PKG_AUTHORS");
			if authors.is_empty() {
				::std::vec![]
			} else {
				authors.split(':').map(|author| author.trim()).collect::<::std::vec::Vec<&'static str>>()
			}
		}
	};
	let init_call = if item.block.stmts.is_empty() {
		zyn! {}
	} else {
		zyn! {
			#[inline]
			async fn init(&self) -> ::puniyu_plugin::Result {
				{{ fn_name }}().await
			}
		}
	};

	zyn! {
		{{ item }}

		pub struct Plugin;

		#[::puniyu_plugin::__private::async_trait]
		impl ::puniyu_plugin::__private::Plugin for Plugin {
			fn name(&self) -> &str {
				#plugin_name
			}

			fn version(&self) -> ::puniyu_plugin::Version {
				::puniyu_plugin::Version {
					major: {{ version_major }},
					minor: {{ version_minor }},
					patch: {{ version_patch }},
				}
			}

			fn author(&self) -> Vec<&'static str> {
				#plugin_author
			}

			fn description(&self) -> Option<&'static str> {
				#plugin_desc
			}

			fn prefix(&self) -> Option<&'static str> {
				#plugin_prefix
			}

			fn tasks(&self) -> Vec<::std::sync::Arc<dyn ::puniyu_plugin::__private::Task>> {
				::puniyu_plugin::__private::inventory::iter::<TaskRegistry>
					.into_iter()
					.filter(|task| task.plugin_name == {{ plugin_name }})
					.map(|task| (task.builder)())
					.collect()
			}

			fn commands(&self) -> Vec<::std::sync::Arc<dyn ::puniyu_plugin::__private::Command>> {
				::puniyu_plugin::__private::inventory::iter::<CommandRegistry>
					.into_iter()
					.filter(|command| command.plugin_name == {{ plugin_name}})
					.map(|command| (command.builder)())
					.collect()
			}

			fn hooks(&self) -> Vec<::std::sync::Arc<dyn ::puniyu_plugin::__private::Hook>> {
				::puniyu_plugin::__private::inventory::iter::<HookRegistry>
					.into_iter()
					.filter(|hook| hook.plugin_name == {{ plugin_name}})
					.map(|hook| (hook.builder)())
					.collect()
			}

			fn config(&self) -> Vec<::std::sync::Arc<dyn ::puniyu_plugin::__private::Config>> {
				::puniyu_plugin::__private::inventory::iter::<ConfigRegistry>
					.into_iter()
					.filter(|config| config.plugin_name == {{ plugin_name}})
					.map(|config| (config.builder)())
					.collect()
			}

			fn server(&self) -> Option<::puniyu_plugin::__private::ServerFunction> {
					@if(cfg.server.is_some()){
						::puniyu_plugin::__private::ServerFunction::new(&cfg.server)
					}
					@else{
						None
					}
			}

			{{ init_call }}
		}

		/// 定时计划注册表
		pub(crate) struct TaskRegistry {
			/// 插件名称
			plugin_name: &'static str,
			/// 任务构造器
			builder: fn() -> ::std::sync::Arc<dyn ::puniyu_plugin::__private::Task>,
		}
		::puniyu_plugin::__private::inventory::collect!(crate::TaskRegistry);

		pub(crate) struct CommandRegistry {
			plugin_name: &'static str,
			/// 命令构造器
			builder: fn() -> ::std::sync::Arc<dyn ::puniyu_plugin::__private::Command>,
		}
		::puniyu_plugin::__private::inventory::collect!(crate::CommandRegistry);

		/// 配置注册表
		pub(crate) struct ConfigRegistry {
			plugin_name: &'static str,
			/// 配置构造器
			builder: fn() -> ::std::sync::Arc<dyn ::puniyu_plugin::__private::Config>,
		}
		::puniyu_plugin::__private::inventory::collect!(crate::ConfigRegistry);

		/// 钩子注册注册表
		pub(crate) struct HookRegistry {
			plugin_name: &'static str,
			/// 钩子构造器
			builder: fn() -> ::std::sync::Arc<dyn ::puniyu_plugin::__private::Hook>,
		}
		::puniyu_plugin::__private::inventory::collect!(crate::HookRegistry);

	}
	.to_token_stream()
}
