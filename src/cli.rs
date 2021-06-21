use clap::{ArgMatches, App, Arg};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const NAME: &str = env!("CARGO_PKG_NAME");

pub fn cli() -> ArgMatches {
    App::new(NAME)
        .version(VERSION)
        .author(AUTHOR)
        .about("[R]ust [R]edis [Top] - tool for monitoring redis server.")
        .arg(Arg::new("host")
            .short('h')
            .about("Server hostname.")
            .default_value("127.0.0.1")
            .required(false)
            .takes_value(true))

        .arg(Arg::new("port")
            .short('p')
            .about("Server port.")
            .default_value("6379")
            .required(false)
            .takes_value(true))
        .arg(Arg::new("socket")
            .short('s')
            .about("Server socket (overrides hostname and port).")
            .required(false)
            .takes_value(true))
        .arg(Arg::new("connection-timeout")
            .short('t')
            .about("Connection timeout in seconds")
            .default_value("5")
            .required(false)
            .takes_value(true))
        .get_matches()
}