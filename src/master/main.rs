use clap::{App, Arg};
use futures::future::Future;
use sha3::{Digest, Sha3_256};
use std::net::{IpAddr, SocketAddr};
use std::path::Path;
use std::{thread, time};
use thesamo::configuration::Config;
use thesamo::extractor::{extract_body_from_tags, FileTags};
use thesamo::file::Files;
use thesamo::{Error, SyncFilePacket};
use tokio::io;
use tokio::net::TcpStream;

#[derive(Debug)]
struct Master {
    files: Vec<Files>,
    address: SocketAddr,
    tags: FileTags,
    polling_freq: u32,
}

impl Master {
    pub fn from_config(config: Config) -> Result<Self, Error> {
        if config.minion {
            // The user needs to double check the config
            return Err(Error::Master(
                "Config file specifies that this is a minion.".to_owned(),
            ));
        } else if config.master {
            let address = SocketAddr::new(
                IpAddr::V4(config.network.bind_address.parse().unwrap()),
                config.network.bind_port,
            );
            let mut master = Self {
                files: config.files,
                address,
                tags: FileTags::new(&config.open_tag, &config.close_tag),
                polling_freq: config.polling_freq.unwrap_or(60),
            };

            match master.check_files() {
                Ok(_) => Ok(master),
                Err(e) => Err(Error::Master(format!(
                    "Error accessing the files for monitoring: [{}]",
                    e
                ))),
            }
        } else {
            Err(Error::Master(
                "Specify master or minion mode in configuration file."
                    .to_owned(),
            ))
        }
    }

    pub fn check_files(&mut self) -> Result<(), Error> {
        for file in self.files.iter_mut() {
            let mut hasher = Sha3_256::new();
            let content = match Files::read_file(&file.path) {
                Ok(c) => c,
                Err(_) => {
                    return Err(Error::Master(format!(
                        "Failed to read: [{}]",
                        &file.path.display()
                    )))
                }
            };
            hasher.input(content.as_bytes());
            file.hash = Some(hasher.result().to_vec());
        }
        Ok(())
    }

    pub fn monitor(&self) {
        for file in &self.files {
            let file_content = Files::read_file(&file.path)
                .expect("File read error in monitor function.");

            let blocks = extract_body_from_tags(&file_content, &self.tags);
            let packet = SyncFilePacket {
                blocks,
                path: file.path.clone(),
            };

            let client = TcpStream::connect(&self.address)
                .and_then(move |stream| {
                    let data_to_send = serde_cbor::to_vec(&packet).unwrap();
                    io::write_all(stream, data_to_send).then(|_| Ok(()))
                })
                .map_err(|err| {
                    println!("connection error = {:?}", err);
                });

            tokio::run(client);
        }
    }
}

fn main() {
    let matches = App::new("thesamo")
        .version("0.1")
        .author("VersBinarii <versbinarii@gmail.com>")
        .about("Config files synchroniser")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Specify config file")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let config_path = matches.value_of("config").unwrap_or("./thesamo.toml");
    let config = Config::new(Path::new(config_path));
    let master = Master::from_config(config).unwrap();

    loop {
        master.monitor();
        let sleep_time = time::Duration::from_secs(master.polling_freq as u64);
        thread::sleep(sleep_time);
    }
}
