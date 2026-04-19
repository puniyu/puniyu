use std::sync::Arc;

use puniyu_loader_ipc::{IpcRegistry, IpcProcess, ProcessState};

fn create_process(name: &str) -> IpcProcess {
	IpcProcess::new(name, format!("/tmp/{name}"))
}

#[test]
fn register_and_get_by_id() {
	let name = format!("ipc-process-register-id-{}", std::process::id());
	let id = IpcRegistry::register(create_process(&name)).expect("register process");

	let process = IpcRegistry::get(id).expect("get process by id");
	assert_eq!(process.id(), id);
	assert_eq!(process.name(), name);

	IpcRegistry::unregister(id).expect("cleanup process");
}

#[test]
fn register_and_get_by_name() {
	let name = format!("ipc-process-register-name-{}", std::process::id());
	let id = IpcRegistry::register(create_process(&name)).expect("register process");

	let process = IpcRegistry::get(name.clone()).expect("get process by name");
	assert_eq!(process.id(), id);
	assert_eq!(process.name(), name);

	IpcRegistry::unregister(id).expect("cleanup process");
}

#[test]
fn reject_duplicate_name() {
	let name = format!("ipc-process-duplicate-{}", std::process::id());
	let id = IpcRegistry::register(create_process(&name)).expect("register process");

	let error = IpcRegistry::register(create_process(&name)).expect_err("duplicate name should fail");
	assert_eq!(error.to_string(), "exists: IpcPluginProcess");

	IpcRegistry::unregister(id).expect("cleanup process");
}

#[test]
fn set_state_updates_runtime() {
	let name = format!("ipc-process-set-state-{}", std::process::id());
	let id = IpcRegistry::register(create_process(&name)).expect("register process");

	IpcRegistry::set_state(id, ProcessState::Running).expect("set state");

	let process = IpcRegistry::get(id).expect("get process after state update");
	assert_eq!(process.state(), ProcessState::Running);

	IpcRegistry::unregister(id).expect("cleanup process");
}

#[test]
fn unregister_by_id_and_name() {
	let name_by_id = format!("ipc-process-unregister-id-{}", std::process::id());
	let id = IpcRegistry::register(create_process(&name_by_id)).expect("register process by id");
	IpcRegistry::unregister(id).expect("unregister by id");
	assert!(IpcRegistry::get(id).is_none());

	let name_by_name = format!("ipc-process-unregister-name-{}", std::process::id());
	let id = IpcRegistry::register(create_process(&name_by_name)).expect("register process by name");
	IpcRegistry::unregister(name_by_name.clone()).expect("unregister by name");
	assert!(IpcRegistry::get(id).is_none());
	assert!(IpcRegistry::get(name_by_name).is_none());
}

#[test]
fn all_returns_registered_processes() {
	let name = format!("ipc-process-all-{}", std::process::id());
	let id = IpcRegistry::register(create_process(&name)).expect("register process");

	let all = IpcRegistry::all();
	assert!(all.iter().any(|process| process.id() == id && process.name() == name));

	IpcRegistry::unregister(id).expect("cleanup process");
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
