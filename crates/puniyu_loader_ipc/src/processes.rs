use std::io::{Error, ErrorKind, Result};
use std::process::Stdio;

use tokio::process::{Child, Command};
use tokio::time::{Duration, timeout};

use crate::protocol::{build_process_meta_request, decode_process_meta_response, protocol_error};
use crate::types::{IpcProcess, ProcessState};

const PROBE_TIMEOUT: Duration = Duration::from_secs(3);

pub async fn probe_process(process: IpcProcess) -> Result<IpcProcess> {
	process.set_state(ProcessState::Starting);

	let server = puniyu_ipc::Server::bind().await?;
	let mut child = spawn_process(&process)?;
	process.set_pid(child.id());

	let probe_result = request_process_meta(&server).await;
	match probe_result {
		Ok(meta) => {
			if meta.name != process.name() {
				process.set_state(ProcessState::Failed);
				shutdown_process(&mut child).await?;
				return Err(Error::new(
					ErrorKind::InvalidData,
					format!("plugin name mismatch: expected {}, got {}", process.name(), meta.name),
				));
			}

			let _ = (&meta.version, &meta.description, &meta.author);
			process.set_state(ProcessState::Running);
			process.set_state(ProcessState::Stopping);
			shutdown_process(&mut child).await?;
			process.set_state(ProcessState::Stopped);
			Ok(process)
		}
		Err(error) => {
			process.set_state(ProcessState::Failed);
			shutdown_process(&mut child).await?;
			Err(error)
		}
	}
}

pub fn spawn_process(process: &IpcProcess) -> Result<Child> {
	let mut command = Command::new(process.program());
	command.args(process.args());
	if let Some(cwd) = process.cwd() {
		command.current_dir(cwd);
	}
	command.envs(process.env());
	command.stdin(Stdio::null());
	command.stdout(Stdio::null());
	command.stderr(Stdio::null());
	command.spawn()
}

pub async fn shutdown_process(child: &mut Child) -> Result<()> {
	if child.id().is_some() {
		let _ = child.kill().await;
	}
	let _ = child.wait().await?;
	Ok(())
}

async fn request_process_meta(
	server: &puniyu_ipc::Server,
) -> Result<crate::types::ProcessMeta> {
	let mut connection = timeout(PROBE_TIMEOUT, server.accept())
		.await
		.map_err(|_| Error::new(ErrorKind::TimedOut, "accept timeout"))??;

	let request = build_process_meta_request()?;
	connection.send(&request).await.map_err(protocol_error)?;

	let response = timeout(PROBE_TIMEOUT, connection.recv())
		.await
		.map_err(|_| Error::new(ErrorKind::TimedOut, "receive timeout"))?
		.map_err(protocol_error)?;

	decode_process_meta_response(response)
}
