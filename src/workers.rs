use std::{io, thread, time};
use std::ops::DerefMut;
use std::thread::JoinHandle;
use std::time::Duration;

use crossterm::event::{KeyCode, KeyModifiers};
use crossterm::event;
use crossterm::event::Event::Key;
use flume::{Receiver, Sender};
use log::error;
use r2d2::Pool;
use redis::{Client, Connection, RedisError};

use crate::event::{AppEvent, RedisRequest, RedisResult};
use crate::metric::Metric;
use crate::metric::slow_log::SlowLog;

pub fn setup_terminal_worker(tx: Sender<AppEvent>) -> io::Result<JoinHandle<()>> {
    thread::Builder::new().name("terminal-events".into()).spawn(move || loop {
        if let Ok(event) = event::read() {
            if let Key(key) = event {
                if key.code == KeyCode::Char('q') || (key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c')) {
                    if let Err(e) = tx.send(AppEvent::Terminate) {
                        error!("{}", e);
                    }
                    return;
                }
            }

            if let Err(e) = tx.send(AppEvent::Terminal(event)) {
                error!("{}", e);
            }
        }
    })
}

pub fn setup_tick_worker(tx: Sender<AppEvent>, tick_rate: Duration) -> io::Result<JoinHandle<()>> {
    thread::Builder::new().name("tick-event".into()).spawn(move || loop {
        // println!("Tick {:?}", thread::current().name());
        if let Err(e) = tx.send(AppEvent::Tick) {
            error!("{}", e);
            break;
        }
        thread::sleep(tick_rate);
    })
}

pub fn setup_redis_workers(tx: Sender<AppEvent>, rx: Receiver<AppEvent>, worker_number: usize, pool: Pool<Client>) -> io::Result<Vec<JoinHandle<()>>> {
    let mut workers = Vec::with_capacity(worker_number);

    for i in 0..worker_number {
        let rx = rx.clone();
        let tx = tx.clone();
        let pool = pool.clone();
        let name = format!("redis-worker-{}", i);
        let worker = thread::Builder::new().name(name).spawn(move || {
            //  println!("created {:?}", thread::current().name());
            loop {
                let event = rx.recv().unwrap_or(AppEvent::Terminate);
                match event {
                    AppEvent::Request(request) => {
                        let p = &mut pool.get();
                        let client = match p {
                            Ok(c) => c.deref_mut(),
                            Err(e) => {
                                error!("{}", e);
                                continue;// or break?
                            }
                        };

                        let result = match request {
                            RedisRequest::Info => {
                                match info(client) {
                                    Ok(result) => { result }
                                    Err(e) => {
                                        error!("{}", e);
                                        continue;// or break?
                                    }
                                }
                            }
                            RedisRequest::SlowLog => {
                                match slow_log(client) {
                                    Ok(result) => { result }
                                    Err(e) => {
                                        error!("{}", e);
                                        continue;// or break?
                                    }
                                }
                            }
                        };
                        if let Err(e) = tx.send(result) {
                            error!("{}", e);
                        }
                    }
                    AppEvent::Terminate => {
                        break;
                    }
                    _ => {}
                };
            }
        })?;
        workers.push(worker);
    }
    Ok(workers)
}

fn info(client: &mut Connection) -> Result<AppEvent, RedisError> {
    let start = time::Instant::now();
    let info = redis::cmd("info").arg("all").query::<String>(client)?;
    let latency = start.elapsed().as_millis();

    Ok(AppEvent::Result(RedisResult::Info(Metric::from(info).latency(latency))))
}

fn slow_log(client: &mut Connection) -> Result<AppEvent, RedisError> {
    let v = redis::cmd("slowlog").arg("get").arg("50").query::<Vec<Vec<(u64, i64, i64, Vec<String>, String, String)>>>(client)?;
    Ok(AppEvent::Result(RedisResult::SlowLog(SlowLog::from(v))))
}