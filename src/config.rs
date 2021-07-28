use clap::ArgMatches;
use tui::style::Style;

use crate::colorscheme::theme::Theme;
use crate::error::RRTopError;

const MIN_WIDTH: u16 = 60;
const MIN_HEIGHT: u16 = 13;
const MIN_TICK_RATE: f64 = 0.5;
const MIN_WORKER_NUMBER: usize = 1;
const MAX_WORKER_NUMBER: usize = 10;

pub struct Config {
    pub timeout: u64,
    pub url: String,
    pub worker_number: usize,
    pub tick_rate: f64,
    pub theme: Theme,
    pub draw_background: Option<Style>,
    pub min_width: u16,
    pub min_height: u16,
    pub file_log_path: Option<String>,
}

impl Config {
    pub fn parse(matches: ArgMatches) -> Result<Config, RRTopError> {
        let host = matches.value_of("host").unwrap().to_owned();
        let port = matches.value_of("port").unwrap().parse::<u16>()?;

        let username = matches.value_of("username");
        let password = matches.value_of("password");

        let url = if let Some(socket) = matches.value_of("socket") {
            let auth = if let Some(pass) = password {
                format!("pass={}&user={}", pass, username.unwrap_or(""))
            } else {
                format!("")
            };

            format!("redis+unix:///{}?{}", socket, auth)
        } else {
            let auth = if let Some(pass) = password {
                format!("{}:{}@", username.unwrap_or(""), pass)
            } else {
                format!("")
            };

            format!("redis://{}{}:{}/", auth, host, port)
        };

        let timeout = matches.value_of("connection-timeout").unwrap().parse::<u64>()?;

        let tick_rate = matches.value_of("tick-rate").unwrap().parse::<f64>()?;
        if tick_rate < MIN_TICK_RATE {
            return Err(RRTopError::cli_parse_error(format!("Tick rate to low. Min value {}", MIN_TICK_RATE)));
        }

        let worker_number = matches.value_of("worker-number").unwrap().parse::<usize>()?;
        if worker_number < MIN_WORKER_NUMBER {
            return Err(RRTopError::cli_parse_error(format!("Worker number to low. Min value {}", MIN_WORKER_NUMBER)));
        }
        if worker_number > MAX_WORKER_NUMBER {
            return Err(RRTopError::cli_parse_error(format!("Worker number to high. Max value {}", MAX_WORKER_NUMBER)));
        }

        let theme: Theme = matches.value_of("color-scheme").unwrap().into();

        let draw_background = match matches.value_of("draw-background").unwrap().parse::<bool>()? {
            true => { Some(theme.main) }
            false => { None }
        };

        let file_log_path = if let Some(file_log_path) = matches.value_of("file-log-path") {
            Some(file_log_path.to_owned())
        } else {
            None
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
            file_log_path,
        })
    }
}