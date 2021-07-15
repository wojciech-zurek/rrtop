use std::{io, thread, time};
use std::ops::DerefMut;
use std::thread::JoinHandle;
use std::time::Duration;

use crossterm::event::{KeyCode, KeyModifiers};
use crossterm::event;
use crossterm::event::Event::Key;
use flume::{Receiver, Sender};
use r2d2::Pool;
use redis::Client;

use crate::event::AppEvent;
use crate::metric::Metric;
use crate::response::Info;

pub fn setup_terminal_worker(tx: Sender<AppEvent>) -> io::Result<JoinHandle<()>> {
    thread::Builder::new().name("terminal-events".into()).spawn(move || loop {
        if let Ok(event) = event::read() {
            if let Key(key) = event {
                if key.code == KeyCode::Char('q') || (key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c')) {
                    if let Err(e) = tx.send(AppEvent::Terminate) {
                        eprintln!("{}", e); //todo: log error
                    }
                    return;
                }
            }

            if let Err(e) = tx.send(AppEvent::Terminal(event)) {
                eprintln!("{}", e);//todo: log error
            }
        }
    })
}

pub fn setup_tick_worker(tx: Sender<AppEvent>, tick_rate: Duration) -> io::Result<JoinHandle<()>> {
    thread::Builder::new().name("tick-event".into()).spawn(move || loop {
        // println!("Tick {:?}", thread::current().name());
        if let Err(e) = tx.send(AppEvent::Tick) {
            eprintln!("{}", e);//todo: log error
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
                    AppEvent::Command => {
                        let p = &mut pool.get();
                        let client = match p {
                            Ok(c) => c.deref_mut(),
                            Err(e) => {
                                eprintln!("{}", e);//todo: log error
                                continue;// or break?
                            }
                        };

                        let start = time::Instant::now();
                        match redis::cmd("INFO").arg("all").query::<Info>(client) {
                            Ok(info) => {
                                let latency = start.elapsed().as_millis();
                                if let Err(e) = tx.send(
                                    AppEvent::Result(Metric::from(info).latency(latency))
                                ) {
                                    eprintln!("{}", e);//todo: log error
                                    // break;
                                }
                            }
                            Err(e) => {
                                eprintln!("{}", e);//todo: log error
                                //break;// ?
                            }
                        };
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