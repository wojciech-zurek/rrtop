use crossterm::event::{Event, KeyCode, KeyModifiers};
use std::{thread, io};
use crossterm::event::Event::Key;
use crossterm::event;
use flume::{RecvError, Sender, Receiver, SendError};
use std::time::Duration;
use std::thread::{JoinHandle, sleep};
use redis::Client;
use crate::config::Config;
use crate::response::Info;

pub enum AppEvent {
    Terminal(Event),
    Tick,
    Terminate,
    Command,
    Result,
}

pub struct Events {
    rx: flume::Receiver<AppEvent>,
    redis_tx: Sender<AppEvent>,
    terminal_worker: JoinHandle<()>,
    tick_worker: JoinHandle<()>,
    redis_workers: Vec<JoinHandle<()>>,
}

impl Events {
    pub fn with_config(config: Config, client: Client) -> io::Result<Events> {
        let (tx, rx) = flume::unbounded();
        let terminal_worker = Events::setup_terminal_events(tx.clone())?;
        let tick_worker = Events::setup_tick_event(tx.clone(), Duration::from_secs(config.tick_rate))?;

        let (redis_tx, redis_rx) = flume::unbounded();
        let redis_workers = Events::setup_redis_workers(tx, redis_rx, config.worker_number, client)?;

        Ok(Events {
            rx,
            redis_tx,
            terminal_worker,
            tick_worker,
            redis_workers,
        })
    }

    fn setup_terminal_events(tx: Sender<AppEvent>) -> io::Result<JoinHandle<()>> {
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

    fn setup_tick_event(tx: Sender<AppEvent>, tick_rate: Duration) -> io::Result<JoinHandle<()>> {
        thread::Builder::new().name("tick-event".into()).spawn(move || loop {
            // println!("Tick {:?}", thread::current().name());
            if let Err(e) = tx.send(AppEvent::Tick) {
                eprintln!("{}", e);//todo: log error
                break;
            }
            thread::sleep(tick_rate);
        })
    }

    fn setup_redis_workers(tx: Sender<AppEvent>, rx: Receiver<AppEvent>, worker_number: usize, client: Client) -> io::Result<Vec<JoinHandle<()>>> {
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
                            // println!("Incoming command {:?}", thread::current().name());
                            // sleep(Duration::from_secs(2));
                            // println!("Done command {:?}", thread::current().name());
                            match redis::cmd("INFO").query::<Info>(&mut client) {
                                Ok(info) => {
                                    //todo send result
                                    if let Err(e) = tx.send(AppEvent::Result) {
                                        eprintln!("{}", e);//todo: log error
                                        break;
                                    }
                                }
                                Err(e) => {
                                    eprintln!("{}", e);//todo: log error
                                    break;// ?
                                }
                            };
                        }
                        AppEvent::Terminate => {
                            // println!("Terminate {:?}", thread::current().name());
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