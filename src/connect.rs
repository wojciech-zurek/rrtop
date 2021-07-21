use std::time::Duration;

use r2d2::Pool;
use redis::Client;

use crate::config::Config;
use crate::error::RRTopError;

pub fn connect(config: &Config) -> Result<Pool<Client>, RRTopError> {
    let client = Client::open(&*config.url)?;

    let pool = r2d2::Pool::builder()
        .connection_timeout(Duration::from_secs(config.timeout))
        .test_on_check_out(false)
        .min_idle(Some(config.worker_number as u32))
        .max_size(config.worker_number as u32).build(client)?;

    Ok(pool)
}
