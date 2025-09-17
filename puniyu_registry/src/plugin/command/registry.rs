use super::builder::CommandBuilder;

pub struct CommandRegistry {
	pub plugin_name: &'static str,
	pub builder: fn() -> Box<dyn CommandBuilder>,
}
