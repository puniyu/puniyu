#[cfg(feature = "command")]
mod command;

#[cfg(feature = "command")]
pub use command::command;
#[cfg(feature = "task")]
mod task;
#[cfg(feature = "task")]
pub use task::task;
#[cfg(feature = "config")]
mod config;
#[cfg(feature = "config")]
pub use config::config;
#[cfg(feature = "server")]
mod server;
#[cfg(feature = "server")]
pub use server::server;
#[cfg(feature = "hook")]
mod hook;
#[cfg(feature = "hook")]
pub use hook::hook;

#[cfg(feature = "plugin")]
#[derive(Debug, Default, darling::FromMeta)]
struct PluginArg {
	pub desc: Option<String>,
	pub prefix: Option<String>,
}

#[cfg(feature = "plugin")]
pub fn plugin(
	args: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	use darling::ast::NestedMeta;
	use darling::{Error, FromMeta};
	use proc_macro2::Ident;
	use quote::quote;
	use syn::{ItemFn, parse_macro_input};
	let attr_args = match NestedMeta::parse_meta_list(args.into()) {
		Ok(v) => v,
		Err(e) => return proc_macro::TokenStream::from(Error::from(e).write_errors()),
	};
	let item = parse_macro_input!(item as ItemFn);
	let fn_name = &item.sig.ident;
	let fn_vis = &item.vis;
	let fn_sig = &item.sig;
	let fn_block = &item.block;
	let is_async = fn_sig.asyncness.is_some();
	if !is_async {
		return syn::Error::new_spanned(fn_sig, "Function must be async").to_compile_error().into();
	}

	let args = match PluginArg::from_list(&attr_args) {
		Ok(v) => v,
		Err(e) => return proc_macro::TokenStream::from(e.write_errors()),
	};
	let plugin_desc = match args.desc {
		Some(desc) => quote! { Some(#desc) },
		None => quote! { None },
	};
	let plugin_prefix = match &args.prefix {
		Some(prefix) => quote! { Some(#prefix) },
		None => quote! { None },
	};
	let plugin_name = quote! { env!("CARGO_PKG_NAME") };
	let version_major = quote! { env!("CARGO_PKG_VERSION_MAJOR").parse::<u16>().unwrap() };
	let version_minor = quote! { env!("CARGO_PKG_VERSION_MINOR").parse::<u16>().unwrap() };
	let version_patch = quote! { env!("CARGO_PKG_VERSION_PATCH").parse::<u16>().unwrap() };
	let version_string = quote! { env!("CARGO_PKG_VERSION") };
	let plugin_author = quote! {
		{
			let authors = env!("CARGO_PKG_AUTHORS");
			if authors.is_empty() {
				None
			} else {
				let first_author = authors.split(':').next().unwrap_or(authors);
				Some(first_author)
			}
		}
	};
	let struct_name = Ident::new("Plugin", fn_name.span());
	let init_call = if fn_block.stmts.is_empty() {
		quote! {
			async {
				puniyu_plugin::logger::info!(
					"{} v{} 初始化完成",
					#plugin_name,
					#version_string,
				);
				Ok(())
			}
		}
	} else {
		quote! {
			async {
				#fn_name().await
			}
		}
	};

	let expanded = quote! {
		pub struct #struct_name;

		#fn_vis #fn_sig #fn_block

		#[::puniyu_plugin::private::async_trait]
		impl ::puniyu_plugin::private::PluginBuilder for #struct_name {
			fn name(&self) -> &str {
				#plugin_name
			}

			fn version(&self) -> ::puniyu_plugin::private::Version {
				::puniyu_plugin::private::Version {
					major: #version_major,
					minor: #version_minor,
					patch: #version_patch,
				}
			}

			fn author(&self) -> Option<&str> {
				#plugin_author
			}

			fn abi_version(&self) -> ::puniyu_plugin::private::Version {
				::puniyu_plugin::private::ABI_VERSION
			}

			fn description(&self) -> Option<&str> {
				#plugin_desc
			}

			fn prefix(&self) -> Option<&str> {
				#plugin_prefix
			}

			fn tasks(&self) -> Vec<Box<dyn ::puniyu_plugin::private::TaskBuilder>> {
				let plugin_name = self.name();
				::puniyu_plugin::private::inventory::iter::<TaskRegistry>
					.into_iter()
					.filter(|task| task.plugin_name == plugin_name)
					.map(|task| (task.builder)())
					.collect()
			}

			fn commands(&self) -> Vec<Box<dyn ::puniyu_plugin::private::CommandBuilder>> {
				let plugin_name = self.name();
				::puniyu_plugin::private::inventory::iter::<CommandRegistry>
					.into_iter()
					.filter(|command| command.plugin_name == plugin_name)
					.map(|command| (command.builder)())
					.collect()
			}

			fn hooks(&self) -> Vec<Box<dyn ::puniyu_plugin::private::HookBuilder>> {
				let plugin_name = self.name();
				::puniyu_plugin::private::inventory::iter::<HookRegistry>
					.into_iter()
					.filter(|hook| hook.plugin_name == plugin_name)
					.map(|hook| (hook.builder)())
					.collect()
			}

			fn config(&self) -> Vec<Box<dyn ::puniyu_plugin::private::Config>> {
				let plugin_name = self.name();
				::puniyu_plugin::private::inventory::iter::<ConfigRegistry>
					.into_iter()
					.filter(|config| config.plugin_name == plugin_name)
					.map(|config| (config.builder)())
					.collect()
			}

			fn server(&self) -> Option<::puniyu_plugin::private::ServerType> {
				let plugin_name = self.name();
				let servers: Vec<_> = ::puniyu_plugin::private::inventory::iter::<ServerRegistry>
					.into_iter()
					.filter(|server| server.plugin_name == plugin_name)
					.map(|server| (server.builder)())
					.collect();

				if !servers.is_empty() {
					Some(::std::sync::Arc::new(move |cfg: &mut ::puniyu_plugin::private::ServiceConfig| {
						servers.iter().for_each(|server| server(cfg));
					}))
				} else {
					None
				}
			}

			async fn init(&self) -> ::puniyu_plugin::private::HandlerResult {
				#init_call.await
			}
		}

		/// 插件注册表
		pub(crate) struct PluginRegistry {
			/// 插件构造器
			builder: fn() -> Box<dyn ::puniyu_plugin::private::PluginBuilder>,
		}
		::puniyu_plugin::private::inventory::collect!(PluginRegistry);

		/// 定时计划注册表
		pub(crate) struct TaskRegistry {
			/// 插件名称
			plugin_name: &'static str,
			/// 任务构造器
			builder: fn() -> Box<dyn ::puniyu_plugin::private::TaskBuilder>,
		}
		::puniyu_plugin::private::inventory::collect!(TaskRegistry);

		pub(crate) struct CommandRegistry {
			plugin_name: &'static str,
			/// 命令构造器
			builder: fn() -> Box<dyn ::puniyu_plugin::private::CommandBuilder>,
		}
		::puniyu_plugin::private::inventory::collect!(CommandRegistry);

		/// 配置注册表
		pub(crate) struct ConfigRegistry {
			plugin_name: &'static str,
			/// 配置构造器
			builder: fn() -> Box<dyn ::puniyu_plugin::private::Config>,
		}
		::puniyu_plugin::private::inventory::collect!(ConfigRegistry);

		/// 服务器注册表
		pub(crate) struct ServerRegistry {
			/// 插件名称
			plugin_name: &'static str,
			/// 服务器配置构造器
			builder: fn() -> ::puniyu_plugin::private::ServerType,
		}
		::puniyu_plugin::private::inventory::collect!(ServerRegistry);

		::puniyu_plugin::private::inventory::submit! {
			crate::PluginRegistry {
				builder: || -> Box<dyn ::puniyu_plugin::private::PluginBuilder> { Box::new(#struct_name {}) },
			}
		}

		/// 钩子注册注册表
		pub(crate) struct HookRegistry {
			plugin_name: &'static str,
			/// 钩子构造器
			builder: fn() -> Box<dyn ::puniyu_plugin::private::HookBuilder>,
		}
		::puniyu_plugin::private::inventory::collect!(HookRegistry);

		#[cfg(feature = "cdylib")]
		#[unsafe(no_mangle)]
		pub unsafe extern "C" fn plugin_info() -> *mut dyn ::puniyu_plugin::private::PluginBuilder {
			Box::into_raw(Box::new(#struct_name {}))
		}

		#[cfg(feature = "cdylib")]
		#[unsafe(no_mangle)]
		pub unsafe extern "C" fn setup_logger(logger: &::puniyu_plugin::logger::SharedLogger) {
			::puniyu_plugin::logger::setup_shared_logger(logger);
		}

		#[cfg(feature = "cdylib")]
		#[unsafe(no_mangle)]
		pub unsafe extern "C" fn setup_app_name(name: String) {
			::puniyu_plugin::common::APP_NAME.get_or_init(|| name);
		}

	};

	expanded.into()
}
