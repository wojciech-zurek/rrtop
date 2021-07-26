use clap::{App, Arg, ArgMatches};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const NAME: &str = env!("CARGO_PKG_NAME");
const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_PORT: &str = "6379";
const DEFAULT_CONNECTION_TIMEOUT: &str = "5";
const DEFAULT_TICK_RATE: &str = "2.0";
const DEFAULT_WORKER_NUMBER: &str = "1";
const DEFAULT_COLOR_SCHEME: &str = "default";
const TRUE: &str = "true";

pub fn cli() -> ArgMatches {
    App::new(NAME)
        .version(VERSION)
        .author(AUTHOR)
        .about("[R]ust [R]edis [Top] - tool for monitoring redis server.")
        .arg(Arg::new("host")
            .short('h')
            .about("Server hostname.")
            .default_value(DEFAULT_HOST)
            .required(false)
            .takes_value(true))
        .arg(Arg::new("port")
            .short('p')
            .about("Server port.")
            .default_value(DEFAULT_PORT)
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
            .default_value(DEFAULT_CONNECTION_TIMEOUT)
            .required(false)
            .takes_value(true))
        .arg(Arg::new("tick-rate")
            .short('r')
            .about("Tick rate in seconds. Be careful.")
            .default_value(DEFAULT_TICK_RATE)
            .required(false)
            .takes_value(true))
        .arg(Arg::new("worker-number")
            .short('w')
            .about("Worker number Be careful.")
            .default_value(DEFAULT_WORKER_NUMBER)
            .required(false)
            .takes_value(true))
        .arg(Arg::new("color-scheme")
            .short('c')
            .about("Color scheme.")
            .possible_values(&["blackbird", "bw", "default", "dracula", "nord", "one-dark", "solarized-dark"])
            .default_value(DEFAULT_COLOR_SCHEME)
            .required(false)
            .takes_value(true))
        .arg(Arg::new("draw-background")
            .short('d')
            .about("Draw background")
            .default_value(TRUE)
            .required(false)
            .takes_value(true))
        .get_matches()
}