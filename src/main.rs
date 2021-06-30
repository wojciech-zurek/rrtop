mod error;
mod cli;
mod response;
mod config;
mod terminal;
mod event;
mod workers;
mod layout;
mod widget;
mod colorscheme;
mod app;
mod update;

use redis::Client;
use error::RRTopError;
use std::time::Duration;
use crate::config::Config;
use cli::cli;
use crate::event::{Events, AppEvent};
use crossterm::event::{Event, KeyCode};
use crate::app::App;
use crate::update::Updatable;

fn main() -> Result<(), RRTopError> {
    let config = config::Config::parse(cli())?;
    let client = connect(&config)?;

    let mut terminal = terminal::create()?;

    let mut events = Events::with_config(&config, client)?;
    let mut app = App::new(&config.color_scheme, config.tick_rate);

    loop {
        layout::draw(&mut terminal, &app)?;
        match events.next()? {
            AppEvent::Terminal(event) => {
                match event {
                    Event::Key(e) => {
                        match e.code {
                            KeyCode::Tab => {app.on_tab()}
                            _ => {}
                        }
                    }
                    Event::Mouse(_) => {}
                    Event::Resize(_, _) => {}
                }
            }
            AppEvent::Tick => {
                events.send(AppEvent::Command)?;
            }
            AppEvent::Terminate => {
                events.terminate();
                break;
            }
            AppEvent::Result(message) => {
                &app.status_bar.update(&message);
                &app.network.update(&message);
                &app.throughput.update(&message);
                &app.cpu.update(&message);
            }
            _ => {}
        }
    }

    // let result: Info = redis::cmd("INFO").query(&mut client)?;
    // println!("{:?}", result);

    terminal::clean(terminal)?;
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


