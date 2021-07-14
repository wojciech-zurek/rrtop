use clap::ArgMatches;
use tui::style::Style;

use crate::cli::DEFAULT_MIN_TICK_RATE;
use crate::colorscheme::theme::Theme;
use crate::error::RRTopError;

const MIN_WIDTH: u16 = 60;
const MIN_HEIGHT: u16 = 13;

pub struct Config {
    pub timeout: u64,
    pub url: String,
    pub worker_number: usize,
    pub tick_rate: f64,
    pub theme: Theme,
    pub draw_background: Option<Style>,
    pub min_width: u16,
    pub min_height: u16,
}

impl Config {
    pub fn parse(matches: ArgMatches) -> Result<Config, RRTopError> {
        let host = matches.value_of("host").unwrap().to_owned();
        let port = matches.value_of("port").unwrap().parse::<u16>()?;
        let timeout = matches.value_of("connection-timeout").unwrap().parse::<u64>()?;
        let tick_rate = matches.value_of("tick-rate").unwrap().parse::<f64>()?;

        if tick_rate < DEFAULT_MIN_TICK_RATE {
            return Err(RRTopError::CliParseError(format!("Tick rate to low. Min value {}", DEFAULT_MIN_TICK_RATE)));
        }

        let worker_number = matches.value_of("worker-number").unwrap().parse::<usize>()?;
        let url = if let Some(socket) = matches.value_of("socket") {
            format!("redis+unix:///{}", socket)
        } else {
            format!("redis://{}:{}/", host, port)
        };

        let theme: Theme = matches.value_of("color-scheme").unwrap().into();

        let draw_background = match matches.value_of("draw-background").unwrap().parse::<bool>()? {
            true => { Some(theme.main) }
            false => { None }
        };

        Ok(Config {
            timeout,
            url,
            worker_number,
            tick_rate,
            theme,
            draw_background,
            min_width: MIN_WIDTH,
            min_height: MIN_HEIGHT,
        })
    }
}