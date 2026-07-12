use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
	#[error("not found: Plugin")]
	NotFound,

	#[error("exists: Plugin")]
	Exists,
}
