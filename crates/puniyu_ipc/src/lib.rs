use std::{io, path::PathBuf};

pub fn socket_path() -> PathBuf {
	puniyu_path::temp_dir().join("puniyu.sock")
}

pub async fn create_runtime() -> io::Result<()>{
    Ok(())
}
