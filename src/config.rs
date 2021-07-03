use clap::ArgMatches;
use crate::error::RRTopError;
use crate::colorscheme::ColorScheme;
use tui::style::Style;

pub struct Config {
    pub timeout: u64,
    pub url: String,
    pub worker_number: usize,
    pub tick_rate: u64,
    pub color_scheme: ColorScheme,
    pub draw_background: Option<Style>,
}

impl Config {
    pub fn parse(matches: ArgMatches) -> Result<Config, RRTopError> {
        let host = matches.value_of("host").unwrap().to_owned();
        let port = matches.value_of("port").unwrap().parse::<u16>()?;
        let timeout = matches.value_of("connection-timeout").unwrap().parse::<u64>()?;
        let tick_rate = matches.value_of("tick-rate").unwrap().parse::<u64>()?;
        let worker_number = matches.value_of("worker-number").unwrap().parse::<usize>()?;
        let url = if let Some(socket) = matches.value_of("socket") {
            format!("redis+unix:///{}", socket)
        } else {
            format!("redis://{}:{}/", host, port)
        };

        let color_scheme: ColorScheme = matches.value_of("color-scheme").unwrap().into();

        let draw_background = match matches.value_of("draw-background").unwrap().parse::<bool>()? {
            true => { Some(color_scheme.main) }
            false => { None }
        };

        Ok(Config {
            timeout,
            url,
            worker_number,
            tick_rate,
            color_scheme,
            draw_background,
        })
    }
}