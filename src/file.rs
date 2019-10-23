use crate::Error;
use serde_derive::Deserialize;
use sha3::{Digest, Sha3_256};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename = "File")]
pub struct ConfigFile {
    pub path: PathBuf,
    pub minion_address: String,
    pub minion_port: u16,
}

#[derive(Debug, Clone)]
pub struct ScanFile {
    pub path: PathBuf,
    pub hash: Option<Vec<u8>>,
}

impl ScanFile {
    pub fn read_file(&self) -> Result<String, Error> {
        let openend_file = match File::open(&self.path) {
            Err(e) => {
                return Err(Error::FileRead {
                    source: e,
                    path: PathBuf::from(&self.path),
                });
            }
            Ok(file) => file,
        };
        let mut buffered_reader = BufReader::new(openend_file);

        let mut file_content = String::new();
        if let Err(e) = buffered_reader.read_to_string(&mut file_content) {
            return Err(Error::FileRead {
                source: e,
                path: PathBuf::from(&self.path),
            });
        }
        Ok(file_content)
    }

    pub fn hash(&mut self, file_content: &str) {
        let mut hasher = Sha3_256::new();
        hasher.input(file_content);

        self.hash = Some(hasher.result().to_vec());
    }
}

impl std::convert::From<ConfigFile> for ScanFile {
    fn from(f: ConfigFile) -> Self {
        Self {
            path: f.path,
            hash: None,
        }
    }
}
