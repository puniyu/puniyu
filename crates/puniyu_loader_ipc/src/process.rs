use std::io::Result;
use std::process::Stdio;

use tokio::process::{Child, Command};

use crate::IpcProcess;


pub fn spawn_process(process: &IpcProcess) -> Result<Child> {
	let mut command = Command::new(process.program());
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
