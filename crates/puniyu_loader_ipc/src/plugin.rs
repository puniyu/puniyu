mod command;
pub(crate) use command::IpcCommand;
mod task;
pub(crate) use task::IpcTask;

use puniyu_version::Version;

use crate::{IpcProcess, ProcessState};

pub(crate) struct IpcPlugin {
	name: String,
}

impl IpcPlugin {
	pub fn new(name: String) -> Self {
		Self { name }
	}
}
impl puniyu_plugin_core::Plugin for IpcPlugin {
	fn name(&self) -> &str {
		&self.name
	}

	fn version(&self) -> Version {
		todo!()
	}

	fn description(&self) -> Option<&str> {
		todo!()
	}
}

pub async fn probe_plugin(process: IpcProcess) -> std::io::Result<IpcProcess> {
	use crate::PluginMetaData;
	use crate::cilent::Client;
	use crate::process::{shutdown_process, spawn_process};
	use crate::service::plugin;
	use std::io::{Error, ErrorKind};

	process.set_state(ProcessState::Starting);

	let mut child = spawn_process(&process)?;
	process.set_pid(child.id());

	let probe_result = Client.send::<_, PluginMetaData>(plugin::META_DATA, ()).await;
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

			process.set_state(ProcessState::Running);
			process.set_state(ProcessState::Stopping);
			shutdown_process(&mut child).await?;
			process.set_state(ProcessState::Stopped);
			Ok(process)
		}
		Err(error) => {
			process.set_state(ProcessState::Failed);
			shutdown_process(&mut child).await?;
			Err(Error::new(ErrorKind::InvalidData, error))
		}
	}
}
