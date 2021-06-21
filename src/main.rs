mod error;
mod cli;
mod response;
mod connection;

use redis::Client;
use error::RRTopError;
use std::time::Duration;
use crate::response::Info;
use crate::connection::Config;

fn main() -> Result<(), RRTopError> {
    let config = connection::Config::parse(cli::cli())?;
    let mut client = connect(&config)?;
    let result: Info = redis::cmd("INFO").query(&mut client)?;

    println!("{:?}", result);

    Ok(())
}

fn connect(config: &Config) -> Result<Client, RRTopError> {
    let client = Client::open(&*config.url)?;
    let mut con = client.get_connection_with_timeout(Duration::from_secs(config.timeout))?;

    let result: String = redis::cmd("PING").query(&mut con)?;
    if result != "PONG" {
        return Err(RRTopError::UnknownQueryRedisError("No PONG response from server!".to_owned()));
    };

    Ok(client)
}


