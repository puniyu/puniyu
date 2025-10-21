use crate::Command;
use crate::store::CommandStore;
use itertools::Itertools;
use std::sync::{Arc, LazyLock, Mutex};

pub static COMMAND_STORE: LazyLock<Arc<Mutex<CommandStore>>> =
	LazyLock::new(|| Arc::new(Mutex::new(CommandStore::new())));

pub struct CommandRegistry;

impl CommandRegistry {
	pub fn insert(builder: Command) {
		COMMAND_STORE.lock().unwrap().insert(builder);
	}

	pub fn remove_with_id(id: u64) {
		COMMAND_STORE.lock().unwrap().remove_with_id(id);
	}

	pub fn remove_with_name(name: &str) {
		COMMAND_STORE.lock().unwrap().remove_with_name(name)
	}

	pub fn remove_with_plugin_name(plugin_name: &str) {
		COMMAND_STORE.lock().unwrap().remove_with_plugin_name(plugin_name)
	}
	pub fn get_with_id(id: u64) -> Option<Arc<Command>> {
		COMMAND_STORE.lock().unwrap().get_with_id(id)
	}

	pub fn get_with_name(name: &str) -> Option<Arc<Command>> {
		COMMAND_STORE.lock().unwrap().get_with_name(name)
	}

	pub fn get_with_plugin(plugin_name: &str, name: &str) -> Option<Arc<Command>> {
		COMMAND_STORE.lock().unwrap().get_with_plugin(plugin_name, name)
	}

	pub fn get_all() -> Vec<Arc<Command>> {
		COMMAND_STORE.lock().unwrap().get_all()
	}
	pub fn get_plugins_with_command(command_name: &str) -> Vec<String> {
		let command_list = COMMAND_STORE.lock().unwrap().get_all();
		command_list
			.iter()
			.filter(|command| command.builder.name() == command_name)
			.sorted_by_key(|command| command.builder.rank())
			.map(|command| command.plugin_name.to_string())
			.collect()
	}
}
