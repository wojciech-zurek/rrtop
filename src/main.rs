use crossterm::event::{Event, KeyCode};

use cli::cli;
use error::RRTopError;

use crate::app::App;
use crate::event::{AppEvent, Events};
use crate::metric::Metric;
use crate::update::Updatable;

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
mod metric;
mod connect;

fn main() -> Result<(), RRTopError> {
    let config = config::Config::parse(cli())?;
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
                events.send(AppEvent::Command)?;
            }
            AppEvent::Terminate => {
                events.terminate();
                break;
            }
            AppEvent::Result(m) => {
                metric = m.calc_delta(metric, config.tick_rate);
                &app.status_bar.update(&metric);
                &app.network.update(&metric);
                &app.cpu.update(&metric);
                &app.memory.update(&metric);
                &app.stat.update(&metric);
                &app.hit_rate.update(&metric);
                &app.calls.update(&metric);
                &app.raw.update(&metric);
            }
            _ => {}
        }
    }

    terminal.clean()?;
    Ok(())
}


