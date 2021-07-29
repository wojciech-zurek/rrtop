use crossterm::event::{Event, KeyCode};

use cli::cli;
use error::RRTopError;

use crate::app::App;
use crate::event::{AppEvent, Events, RedisRequest, RedisResult};
use crate::metric::Metric;
use crate::update::Updatable;

mod error;
mod cli;
mod config;
mod terminal;
mod event;
mod workers;
mod layout;
mod widget;
mod colorscheme;
mod app;
mod update;
mod metric;
mod connect;
mod logger;

fn main() -> Result<(), RRTopError> {
    let config = config::Config::parse(cli())?;

    logger::init_log(&config)?;

    let pool = connect::connect(&config)?;

    let mut terminal = terminal::Terminal::new()?;

    let mut events = Events::from_config(&config, pool)?;
    let mut app = App::new(&config.theme, config.draw_background, config.min_width, config.min_height);

    let mut metric = Metric::default();
    loop {
        layout::draw(&mut *terminal, &mut app)?;
        match events.next()? {
            AppEvent::Terminal(event) => {
                match event {
                    Event::Key(e) => {
                        match e.code {
                            KeyCode::Tab => { app.on_tab() }
                            KeyCode::Up => { app.on_key_up() }
                            KeyCode::Down => { app.on_key_down() }
                            KeyCode::Char('s') => { app.on_sort() }
                            _ => {}
                        }
                    }
                    Event::Mouse(_) => {}
                    Event::Resize(_, _) => { layout::draw(&mut *terminal, &mut app)?; }
                }
            }
            AppEvent::Tick => {
                events.send(AppEvent::Request(RedisRequest::Info))?;
                events.send(AppEvent::Request(RedisRequest::SlowLog))?;
            }
            AppEvent::Terminate => {
                events.terminate();
                break;
            }
            AppEvent::Result(m) => {
                match m {
                    RedisResult::Info(m) => {
                        metric = m.calc_delta(metric, config.tick_rate);
                        &app.status_bar.update(&metric);
                        &app.network.update(&metric);
                        &app.cpu.update(&metric);
                        &app.memory.update(&metric);
                        &app.stat.update(&metric);
                        &app.calls.update(&metric);
                        &app.raw.update(&metric);
                    }
                    RedisResult::SlowLog(log) => {
                        &app.slow_log.update(log);
                    }
                }
            }
            _ => {}
        }
    }

    terminal.clean()?;
    Ok(())
}


