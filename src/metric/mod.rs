use std::borrow::Borrow;

use crate::metric::command::Command;
use crate::metric::cpu::Cpu;
use crate::metric::keyspace::Keyspace;
use crate::metric::memory::Memory;
use crate::metric::server::Server;
use crate::metric::stats::Stats;
use crate::response::Info;

pub mod server;
pub mod cpu;
pub mod memory;
pub mod keyspace;
pub mod command;
pub mod stats;
pub mod slow_log;

#[derive(Default)]
pub struct Metric {
    pub server: Server,
    pub stats: Stats,
    pub cpu: Cpu,
    pub memory: Memory,
    pub keyspace: Keyspace,
    pub command: Command,
    pub raw: Info,
}

impl Metric {
    pub fn latency(mut self, latency: u128) -> Self {
        self.server.latency = latency;
        self
    }

    pub fn calc_delta(mut self, other: Self, tick_rate: f64) -> Self {
        self.cpu.calc_delta(&other.cpu, tick_rate);
        self.stats.calc_delta(&other.stats, tick_rate);
        self
    }
}

impl From<Info> for Metric {
    fn from(i: Info) -> Self {
        Metric {
            server: i.borrow().into(),
            stats: i.borrow().into(),
            cpu: i.borrow().into(),
            memory: i.borrow().into(),
            keyspace: i.borrow().into(),
            command: i.borrow().into(),
            raw: i,
        }
    }
}

trait Delta {
    fn calc_delta(&mut self, other: &Self, tick_rate: f64);
}