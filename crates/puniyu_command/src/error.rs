use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
	#[error("not found: Command")]
	NotFound,

	#[error("exists: Command")]
	Exists,
}
