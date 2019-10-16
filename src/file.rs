use crate::Error;
use serde_derive::Deserialize;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

#[derive(Deserialize, Debug, Clone)]
pub struct Files {
    pub path: PathBuf,
    pub minion_address: String,
    pub minion_port: u16,
    pub hash: Option<Vec<u8>>,
}

impl Files {
    pub fn read_file(path: &Path) -> Result<String, Error> {
        let openend_file = match File::open(path) {
            Err(e) => {
                return Err(Error::FileRead {
                    source: e,
                    path: PathBuf::from(path),
                });
            }
            Ok(file) => file,
        };
        let mut buffered_reader = BufReader::new(openend_file);

        let mut file_content = String::new();
        if let Err(e) = buffered_reader.read_to_string(&mut file_content) {
            return Err(Error::FileRead {
                source: e,
                path: PathBuf::from(path),
            });
        }
        Ok(file_content)
    }
}
