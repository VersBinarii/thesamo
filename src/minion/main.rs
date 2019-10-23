use clap::{App, Arg};
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::net::{IpAddr, SocketAddr};
use std::path::Path;
use thesamo::configuration::Config;
use thesamo::extractor::{replace_in_tags, FileTags};
use thesamo::file::ScanFile;
use thesamo::SyncFilePacket;
use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;

#[derive(Debug, Clone)]
pub struct Minion {
    files: Vec<ScanFile>,
    tags: FileTags,
}

impl Minion {
    pub fn from_config(config: Config) -> Self {
        if config.minion {
            return Self {
                files: config.files.into_iter().map(From::from).collect(),
                tags: FileTags::new(&config.open_tag, &config.close_tag),
            };
        } else {
            panic!("Set the \"minion = true\" in the configuration file.")
        }
    }

    fn handle_packet(self, packet: SyncFilePacket) {
        // Check if the file to be updated is the one
        // we have in our list
        if let Some(file) = self
            .files
            .iter()
            .find(|&e| e.path.file_name() == packet.path.file_name())
        {
            let file_content =
                file.read_file().expect("Failed to read the file");
            let replaced_file =
                replace_in_tags(&file_content, &packet.blocks, &self.tags)
                    .expect("Error replacing the file content.");

            // Write replaced content into the file
            let file_to_write = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(&file.path)
                .unwrap();
            let mut buf_writer = BufWriter::new(file_to_write);
            match buf_writer.write_all(replaced_file.as_bytes()) {
                Ok(_) => {}
                Err(e) => println!("{}", e),
            }
        }
    }
}
pub fn listen(bind_address: &SocketAddr, minion: Minion) {
    let listener = TcpListener::bind(&bind_address).unwrap();

    // accept connections and process them
    tokio::run(
        listener
            .incoming()
            .map_err(|e| eprintln!("failed to accept socket; error = {:?}", e))
            .for_each(move |socket| {
                let minion = minion.clone();
                let buffer = vec![];
                let fut = io::read_to_end(socket, buffer)
                    .and_then(|(_, bytes)| {
                        let packet_from_master: SyncFilePacket =
                            serde_cbor::from_slice(&bytes).unwrap();
                        minion.handle_packet(packet_from_master);
                        Ok(())
                    })
                    .map_err(|_| ());

                tokio::spawn(fut);
                Ok(())
            }),
    );
}

/// The minion receives the updates from its master
/// It will constantly listen for master update messages.
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

    let address = SocketAddr::new(
        IpAddr::V4(config.network.bind_address.parse().unwrap()),
        config.network.bind_port,
    );
    let minion = Minion::from_config(config);
    listen(&address, minion);
}
