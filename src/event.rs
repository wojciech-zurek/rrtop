use std::io;
use std::thread::JoinHandle;
use std::time::Duration;

use crossterm::event::Event;
use flume::{RecvError, Sender, SendError};
use r2d2::Pool;
use redis::Client;

use crate::config::Config;
use crate::metric::Metric;
use crate::metric::slow_log::SlowLog;
use crate::workers::{setup_redis_workers, setup_terminal_worker, setup_tick_worker};

pub enum RedisRequest {
    Info,
    SlowLog,
}

pub enum RedisResult {
    Info(Metric),
    SlowLog(SlowLog),
}

pub enum AppEvent {
    Terminal(Event),
    Tick,
    Terminate,
    Request(RedisRequest),
    Result(RedisResult),
}


pub struct Events {
    rx: flume::Receiver<AppEvent>,
    redis_tx: Sender<AppEvent>,
    _terminal_worker: JoinHandle<()>,
    _tick_worker: JoinHandle<()>,
    redis_workers: Vec<JoinHandle<()>>,
}

impl Events {
    pub fn from_config(config: &Config, pool: Pool<Client>) -> io::Result<Events> {
        let (tx, rx) = flume::unbounded();
        let terminal_worker = setup_terminal_worker(tx.clone())?;
        let tick_worker = setup_tick_worker(tx.clone(), Duration::from_secs_f64(config.tick_rate))?;

        let (redis_tx, redis_rx) = flume::unbounded();
        let redis_workers = setup_redis_workers(tx, redis_rx, config.worker_number, pool)?;

        Ok(Events {
            rx,
            redis_tx,
            _terminal_worker: terminal_worker,
            _tick_worker: tick_worker,
            redis_workers,
        })
    }

    pub fn next(&mut self) -> Result<AppEvent, RecvError> {
        self.rx.recv()
    }

    pub fn send(&self, event: AppEvent) -> Result<(), SendError<AppEvent>> {
        self.redis_tx.send(event)
    }

    pub fn terminate(self) {
        for _ in &self.redis_workers {
            let _ = self.redis_tx.send(AppEvent::Terminate);
        }

        for worker in self.redis_workers {
            let _ = worker.join();
        }
    }
}