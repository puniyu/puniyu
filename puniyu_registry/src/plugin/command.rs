pub mod builder;
mod registry;

#[derive(Debug, Clone)]
pub struct Command {
	pub name: &'static str,
	pub command: &'static str,
	pub rank: usize,
}
