#[macro_use]
extern crate lazy_static;
extern crate serde;

use clap::Parser;

pub mod chord;
pub mod entities;
pub mod error;
pub mod parser;
pub mod song;
pub mod web;

// Setup the command line interface with clap.
#[derive(Parser, Debug)]
#[clap(name = "server", about = "A server for our wasm project!")]
pub struct Opt {
    /// set the log level
    #[clap(short = 'l', long = "log", default_value = "debug")]
    log_level: String,

    /// set the listen addr
    #[clap(short = 'a', long = "addr", default_value = "::1")]
    addr: String,

    /// set the listen port
    #[clap(short = 'p', long = "port", default_value = "8080")]
    port: u16,

    /// set the directory where static files are to be found
    #[clap(long = "static-dir", default_value = "./dist")]
    static_dir: String,
}

impl Opt {
    pub fn log_level(&self) -> &str {
        &self.log_level
    }
}
