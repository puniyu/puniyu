mod command;
pub use command::command;
mod task;
pub use task::task;

use crate::PluginArg;
use quote::quote;
use syn::ItemStruct;

pub fn plugin_struct(item: ItemStruct, cfg: PluginArg) -> proc_macro2::TokenStream {
	if let Err(err) = crate::common::ensure_valid_host(&item.ident, "Plugin") {
		return err.to_compile_error();
	}

	let user_struct_name = &item.ident;

	let plugin_desc = match cfg.desc {
		Some(desc) => quote!(Some(#desc)),
		None => quote!(None),
	};

	quote! {
		#item

		impl #user_struct_name {
			fn __puniyu_description() -> Option<&'static str> {
				#plugin_desc
			}
		}

		pub(crate) struct TaskRegistry {
			pub(crate) plugin_name: &'static str,
			pub(crate) handler: fn() -> ::std::sync::Arc<dyn ::puniyu_plugin::task::Task>,
		}
		impl TaskRegistry {
			pub(crate) fn tasks() -> ::std::vec::Vec<::puniyu_plugin::task::TaskHandle> {
				::puniyu_plugin::inventory::iter::<Self>()
					.filter(|r| r.plugin_name == env!("CARGO_PKG_NAME"))
					.map(|r| ::puniyu_plugin::task::TaskHandle::new((r.handler)()))
					.collect()
			}
		}
		::puniyu_plugin::inventory::collect!(crate::TaskRegistry);

		pub(crate) struct CommandRegistry {
			pub(crate) plugin_name: &'static str,
			pub(crate) handler: fn() -> ::std::sync::Arc<dyn ::puniyu_plugin::command::Command>,
		}
		impl CommandRegistry {
			pub(crate) fn commands() -> ::std::vec::Vec<::puniyu_plugin::command::CommandHandle> {
				::puniyu_plugin::inventory::iter::<Self>()
					.filter(|r| r.plugin_name == env!("CARGO_PKG_NAME"))
					.map(|r| ::puniyu_plugin::command::CommandHandle::new((r.handler)()))
					.collect()
			}
		}
		::puniyu_plugin::inventory::collect!(crate::CommandRegistry);

		pub(crate) struct ConfigRegistry {
			pub(crate) plugin_name: &'static str,
			pub(crate) handler: fn() -> ::std::vec::Vec<::std::sync::Arc<dyn ::puniyu_plugin::config::Config>>,
		}
		impl ConfigRegistry {
			pub(crate) fn get() -> ::std::vec::Vec<::std::sync::Arc<dyn ::puniyu_plugin::config::Config>> {
				::puniyu_plugin::inventory::iter::<Self>()
					.filter(|r| r.plugin_name == env!("CARGO_PKG_NAME"))
					.flat_map(|r| (r.handler)())
					.collect()
			}
		}
		::puniyu_plugin::inventory::collect!(crate::ConfigRegistry);

		pub(crate) struct OnLoadRegistry {
			pub(crate) handler: fn() -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ::puniyu_plugin::result::Result> + Send>>,
		}
		impl OnLoadRegistry {
			pub(crate) async fn execute() -> ::puniyu_plugin::result::Result {
				if let Some(entry) = ::puniyu_plugin::inventory::iter::<Self>().next() {
					(entry.handler)().await?;
				}
				Ok(())
			}
		}
		::puniyu_plugin::inventory::collect!(crate::OnLoadRegistry);

		pub(crate) struct OnUnloadRegistry {
			pub(crate) handler: fn() -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ::puniyu_plugin::result::Result> + Send>>,
		}
		impl OnUnloadRegistry {
			pub(crate) async fn execute() -> ::puniyu_plugin::result::Result {
				if let Some(entry) = ::puniyu_plugin::inventory::iter::<Self>().next() {
					(entry.handler)().await?;
				}
				Ok(())
			}
		}
		::puniyu_plugin::inventory::collect!(crate::OnUnloadRegistry);

		pub(crate) struct ServerRegistry {
			pub(crate) handler: fn() -> Option<::puniyu_plugin::server::ServerFunction>,
		}
		impl ServerRegistry {
			pub(crate) fn get() -> Option<::puniyu_plugin::server::ServerFunction> {
				::puniyu_plugin::inventory::iter::<Self>()
					.next()
					.and_then(|r| (r.handler)())
			}
		}
		::puniyu_plugin::inventory::collect!(crate::ServerRegistry);

		macro_rules! __puniyu_submit {
			(on_load, $fn:ident) => {
				::puniyu_plugin::inventory::submit! {
					crate::OnLoadRegistry {
						handler: || -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ::puniyu_plugin::result::Result> + Send>> {
							Box::pin($fn())
						}
					}
				}
			};
			(on_unload, $fn:ident) => {
				::puniyu_plugin::inventory::submit! {
					crate::OnUnloadRegistry {
						handler: || -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ::puniyu_plugin::result::Result> + Send>> {
							Box::pin($fn())
						}
					}
				}
			};
			(server, $fn:ident) => {
				::puniyu_plugin::inventory::submit! {
					crate::ServerRegistry { handler: $fn }
				}
			};
			(config, $fn:ident) => {
				::puniyu_plugin::inventory::submit! {
					crate::ConfigRegistry {
						plugin_name: env!("CARGO_PKG_NAME"),
						handler: $fn
					}
				}
			};
			(task, $struct_name:ident) => {
				::puniyu_plugin::inventory::submit! {
					crate::TaskRegistry {
						plugin_name: env!("CARGO_PKG_NAME"),
						handler: || -> ::std::sync::Arc<dyn ::puniyu_plugin::task::Task> {
							::std::sync::Arc::new($struct_name {})
						}
					}
				}
			};
			(command, $struct_name:ident) => {
				::puniyu_plugin::inventory::submit! {
					crate::CommandRegistry {
						plugin_name: env!("CARGO_PKG_NAME"),
						handler: || -> ::std::sync::Arc<dyn ::puniyu_plugin::command::Command> {
							::std::sync::Arc::new($struct_name {})
						}
					}
				}
			};
		}
		pub(crate) use __puniyu_submit;

		pub struct Plugin;

		#[::puniyu_plugin::async_trait]
		impl ::puniyu_plugin::Plugin for Plugin {
			#[inline]
			fn name(&self) -> &str {
				env!("CARGO_PKG_NAME")
			}

			#[inline]
			fn version(&self) -> ::puniyu_plugin::Version {
				::puniyu_plugin::Version {
					major: env!("CARGO_PKG_VERSION_MAJOR").parse::<u64>().unwrap(),
					minor: env!("CARGO_PKG_VERSION_MINOR").parse::<u64>().unwrap(),
					patch: env!("CARGO_PKG_VERSION_PATCH").parse::<u64>().unwrap(),
				}
			}

			#[inline]
			fn description(&self) -> Option<&str> {
				#user_struct_name::__puniyu_description()
			}

			#[inline]
			fn tasks(&self) -> ::std::vec::Vec<::puniyu_plugin::task::TaskHandle> {
				crate::TaskRegistry::tasks()
			}

			#[inline]
			fn commands(&self) -> ::std::vec::Vec<::puniyu_plugin::command::CommandHandle> {
				crate::CommandRegistry::commands()
			}

			#[inline]
			fn server(&self) -> Option<::puniyu_plugin::server::ServerFunction> {
				crate::ServerRegistry::get()
			}

			#[inline]
			fn config(&self) -> ::std::vec::Vec<::std::sync::Arc<dyn ::puniyu_plugin::config::Config>> {
				crate::ConfigRegistry::get()
			}

			#[inline]
			async fn on_load(&self) -> ::puniyu_plugin::result::Result {
				crate::OnLoadRegistry::execute().await
			}

			#[inline]
			async fn on_unload(&self) -> ::puniyu_plugin::result::Result {
				crate::OnUnloadRegistry::execute().await
			}
		}
	}
}
