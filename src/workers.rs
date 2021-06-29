use crossterm::event::{KeyCode, KeyModifiers};
use flume::{Sender, Receiver};
use crate::event::{AppEvent, Message};
use std::{io, thread, time};
use std::thread::JoinHandle;
use crossterm::event::Event::Key;
use crossterm::event;
use std::time::Duration;
use redis::Client;
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

pub fn setup_redis_workers(tx: Sender<AppEvent>, rx: Receiver<AppEvent>, worker_number: usize, client: Client) -> io::Result<Vec<JoinHandle<()>>> {
    let mut workers = Vec::with_capacity(worker_number);

    for i in 0..worker_number {
        let rx = rx.clone();
        let tx = tx.clone();
        let mut client = client.clone();
        let name = format!("redis-worker-{}", i);
        let worker = thread::Builder::new().name(name).spawn(move || {
            //  println!("created {:?}", thread::current().name());
            loop {
                let event = rx.recv().unwrap_or(AppEvent::Terminate);
                match event {
                    AppEvent::Command => {
                        let start = time::Instant::now();
                        match redis::cmd("INFO").query::<Info>(&mut client) {
                            Ok(info) => {
                                if let Err(e) = tx.send(
                                    AppEvent::Result(Message::new(info, start.elapsed().as_millis()))
                                ) {
                                    eprintln!("{}", e);//todo: log error
                                    break;
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