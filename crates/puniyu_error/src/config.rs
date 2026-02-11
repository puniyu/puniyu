use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error")]
    Io(#[from] io::Error),
    #[error("TOML parse error")]
    Deserialize(#[from] toml::de::Error),
    #[error("TOML serialize error")]
    Serialize(#[from] toml::ser::Error),
}

