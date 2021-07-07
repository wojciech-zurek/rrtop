use crossterm::event::Event;
use std::io;
use flume::{RecvError, Sender, SendError};
use std::time::Duration;
use std::thread::JoinHandle;
use redis::Client;
use crate::config::Config;
use crate::response::Info;
use crate::workers::{setup_terminal_worker, setup_tick_worker, setup_redis_workers};
use r2d2::Pool;
use crate::metric::Metric;

pub enum AppEvent {
    Terminal(Event),
    Tick,
    Terminate,
    Command,
    Result(Metric),
}


pub struct Events {
    rx: flume::Receiver<AppEvent>,
    redis_tx: Sender<AppEvent>,
    terminal_worker: JoinHandle<()>,
    tick_worker: JoinHandle<()>,
    redis_workers: Vec<JoinHandle<()>>,
}

impl Events {
    pub fn from_config(config: &Config, pool: Pool<Client>) -> io::Result<Events> {
        let (tx, rx) = flume::unbounded();
        let terminal_worker = setup_terminal_worker(tx.clone())?;
        let tick_worker = setup_tick_worker(tx.clone(), Duration::from_secs(config.tick_rate))?;

        let (redis_tx, redis_rx) = flume::unbounded();
        let redis_workers = setup_redis_workers(tx, redis_rx, config.worker_number, pool)?;

        Ok(Events {
            rx,
            redis_tx,
            terminal_worker,
            tick_worker,
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