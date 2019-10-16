use serde_derive::{Deserialize, Serialize};
use std::path::PathBuf;

pub mod configuration;
pub mod extractor;
pub mod file;

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncFilePacket {
    pub path: PathBuf,
    pub blocks: Vec<String>,
}

#[derive(Debug)]
pub enum Error {
    FileRead {
        source: std::io::Error,
        path: PathBuf,
    },
    FileWrite {
        source: std::io::Error,
        path: PathBuf,
    },
    ReplaceTags(String),
    Master(String),
    Minion(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::FileRead { source, path } => write!(
                f,
                "Error reading from: [{}] - [{}]",
                source,
                path.display()
            ),
            Error::FileWrite { source, path } => write!(
                f,
                "Error writing to: [{}] - [{}]",
                source,
                path.display()
            ),
            Error::ReplaceTags(e) => {
                write!(f, "Failed to replace the tags content: [{}]", e)
            }
            Error::Master(e) => write!(f, "Master error: [{}]", e),
            Error::Minion(e) => write!(f, "Minion error: [{}]", e),
        }
    }
}

impl std::error::Error for Error {}
