use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdapterLifecyclePhase {
	Validation,
	Load,
	Start,
	Stop,
	Unload,
}

impl std::fmt::Display for AdapterLifecyclePhase {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Validation => f.write_str("validation"),
			Self::Load => f.write_str("load"),
			Self::Start => f.write_str("start"),
			Self::Stop => f.write_str("stop"),
			Self::Unload => f.write_str("unload"),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterLifecycleFailure {
	pub adapter: String,
	pub phase: AdapterLifecyclePhase,
	pub message: String,
}

/// 插件关闭阶段。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginLifecyclePhase {
	Stop,
	Unload,
}

impl std::fmt::Display for PluginLifecyclePhase {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Stop => f.write_str("stop"),
			Self::Unload => f.write_str("unload"),
		}
	}
}

/// 插件关闭回调失败信息。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginLifecycleFailure {
	pub plugin: String,
	pub phase: PluginLifecyclePhase,
	pub message: String,
}

/// App 初始化、运行和关闭错误。
#[derive(Debug, Error)]
pub enum AppError {
	#[error(transparent)]
	Io(#[from] std::io::Error),

	#[error("loader '{loader}' failed to discover {component}: {source}")]
	LoaderDiscovery {
		loader: String,
		component: &'static str,
		#[source]
		source: Box<dyn std::error::Error + Send + Sync>,
	},

	#[error("duplicate plugin name: '{0}'")]
	DuplicatePlugin(String),

	#[error("plugin '{plugin}' requires core version '{required}', current version is '{current}'")]
	IncompatiblePluginVersion { plugin: String, required: String, current: String },

	#[error("plugin '{plugin}' load failed: {source}; rollback failures: {rollback_failures:?}")]
	PluginLoad {
		plugin: String,
		#[source]
		source: Box<dyn std::error::Error + Send + Sync>,
		rollback_failures: Vec<PluginLifecycleFailure>,
	},

	#[error("plugin '{plugin}' start failed: {source}; rollback failures: {rollback_failures:?}")]
	PluginStart {
		plugin: String,
		#[source]
		source: Box<dyn std::error::Error + Send + Sync>,
		rollback_failures: Vec<PluginLifecycleFailure>,
	},

	#[error("plugin shutdown completed with {} lifecycle failure(s)", .0.len())]
	PluginShutdown(Vec<PluginLifecycleFailure>),

	#[error("adapter shutdown completed with {} lifecycle failure(s)", .0.len())]
	AdapterShutdown(Vec<AdapterLifecycleFailure>),
}
