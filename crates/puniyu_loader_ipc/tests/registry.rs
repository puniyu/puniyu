use std::collections::HashMap;
use std::sync::Arc;

use puniyu_loader_ipc::{IpcPluginRegistry, IpcProcess, ProcessState};

fn create_process(name: &str) -> IpcProcess {
	IpcProcess::new(name, format!("/tmp/{name}"))
		.with_args(vec!["--serve".to_string()])
		.with_env(HashMap::from([("RUST_LOG".to_string(), "info".to_string())]))
		.with_auto_start(true)
}

#[test]
fn register_and_get_by_id() {
	let name = format!("ipc-process-register-id-{}", std::process::id());
	let id = IpcPluginRegistry::register(create_process(&name)).expect("register process");

	let process = IpcPluginRegistry::get(id).expect("get process by id");
	assert_eq!(process.id(), id);
	assert_eq!(process.name(), name);

	IpcPluginRegistry::unregister(id).expect("cleanup process");
}

#[test]
fn register_and_get_by_name() {
	let name = format!("ipc-process-register-name-{}", std::process::id());
	let id = IpcPluginRegistry::register(create_process(&name)).expect("register process");

	let process = IpcPluginRegistry::get(name.clone()).expect("get process by name");
	assert_eq!(process.id(), id);
	assert_eq!(process.name(), name);

	IpcPluginRegistry::unregister(id).expect("cleanup process");
}

#[test]
fn reject_duplicate_name() {
	let name = format!("ipc-process-duplicate-{}", std::process::id());
	let id = IpcPluginRegistry::register(create_process(&name)).expect("register process");

	let error = IpcPluginRegistry::register(create_process(&name)).expect_err("duplicate name should fail");
	assert_eq!(error.to_string(), "exists: IpcPluginProcess");

	IpcPluginRegistry::unregister(id).expect("cleanup process");
}

#[test]
fn set_state_updates_runtime() {
	let name = format!("ipc-process-set-state-{}", std::process::id());
	let id = IpcPluginRegistry::register(create_process(&name)).expect("register process");

	IpcPluginRegistry::set_state(id, ProcessState::Running).expect("set state");

	let process = IpcPluginRegistry::get(id).expect("get process after state update");
	assert_eq!(process.state(), ProcessState::Running);

	IpcPluginRegistry::unregister(id).expect("cleanup process");
}

#[test]
fn unregister_by_id_and_name() {
	let name_by_id = format!("ipc-process-unregister-id-{}", std::process::id());
	let id = IpcPluginRegistry::register(create_process(&name_by_id)).expect("register process by id");
	IpcPluginRegistry::unregister(id).expect("unregister by id");
	assert!(IpcPluginRegistry::get(id).is_none());

	let name_by_name = format!("ipc-process-unregister-name-{}", std::process::id());
	let id = IpcPluginRegistry::register(create_process(&name_by_name)).expect("register process by name");
	IpcPluginRegistry::unregister(name_by_name.clone()).expect("unregister by name");
	assert!(IpcPluginRegistry::get(id).is_none());
	assert!(IpcPluginRegistry::get(name_by_name).is_none());
}

#[test]
fn all_returns_registered_processes() {
	let name = format!("ipc-process-all-{}", std::process::id());
	let id = IpcPluginRegistry::register(create_process(&name)).expect("register process");

	let all = IpcPluginRegistry::all();
	assert!(all.iter().any(|process| process.id() == id && process.name() == name));

	IpcPluginRegistry::unregister(id).expect("cleanup process");
}

#[test]
fn process_runtime_accessors_work() {
	let process = Arc::new(create_process("ipc-process-accessors"));
	assert_eq!(process.state(), ProcessState::Stopped);
	assert_eq!(process.pid(), None);

	process.set_state(ProcessState::Running);
	process.set_pid(Some(42));
	assert_eq!(process.state(), ProcessState::Running);
	assert_eq!(process.pid(), Some(42));
}
