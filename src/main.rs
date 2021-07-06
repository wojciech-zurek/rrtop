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

use redis::{Client, ConnectionLike};
use error::RRTopError;
use std::time::Duration;
use crate::config::Config;
use cli::cli;
use crate::event::{Events, AppEvent};
use crossterm::event::{Event, KeyCode};
use crate::app::App;
use crate::update::Updatable;
use r2d2::{ManageConnection, Pool};

fn main() -> Result<(), RRTopError> {
    let config = config::Config::parse(cli())?;
    let pool = connect(&config)?;

    let mut terminal = terminal::create()?;

    let mut events = Events::from_config(&config, pool)?;
    let mut app = App::new(&config.theme, config.tick_rate, config.draw_background);

    loop {
        layout::draw(&mut terminal, &mut app)?;
        match events.next()? {
            AppEvent::Terminal(event) => {
                match event {
                    Event::Key(e) => {
                        match e.code {
                            KeyCode::Tab => { app.on_tab() }
                            _ => {}
                        }
                    }
                    Event::Mouse(_) => {}
                    Event::Resize(_, _) => { layout::draw(&mut terminal, &mut app)?; }
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
                &app.memory.update(&message);
                &app.stat.update(&message);
            }
            _ => {}
        }
    }

    // let result: Info = redis::cmd("INFO").query(&mut client)?;
    // println!("{:?}", result);

    terminal::clean(terminal)?;
    Ok(())
}

fn connect(config: &Config) -> Result<Pool<Client>, RRTopError> {
    let client = Client::open(&*config.url)?;

    let pool = r2d2::Pool::builder()
        .connection_timeout(Duration::from_secs(config.timeout))
        .min_idle(Some(config.worker_number as u32))
        .max_size(config.worker_number as u32).build(client)?;

    Ok(pool)
}


