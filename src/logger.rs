use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log::LevelFilter;

use crate::config::Config;
use crate::error::AppError;

pub fn init_log(config: &Config) -> Result<(), AppError> {
    if let Some(log_file_path) = &config.file_log_path {
        let logfile = FileAppender::builder()
            .build(log_file_path)?;

        let config = log4rs::config::Config::builder()
            .appender(Appender::builder().build("logfile", Box::new(logfile)))
            .build(Root::builder()
                .appender("logfile")
                .build(LevelFilter::Info))?;

        log4rs::init_config(config)?;
    }

    Ok(())
}