use std::borrow::Borrow;

use crate::metric::command::Command;
use crate::metric::cpu::Cpu;
use crate::metric::keyspace::Keyspace;
use crate::metric::memory::Memory;
use crate::metric::network::Network;
use crate::metric::status::Status;
use crate::metric::throughput::Throughput;
use crate::response::Info;

pub mod status;
pub mod network;
pub mod cpu;
pub mod memory;
pub mod throughput;
pub mod keyspace;
pub mod command;

#[derive(Default)]
pub struct Metric {
    pub status: Status,
    pub network: Network,
    pub cpu: Cpu,
    pub memory: Memory,
    pub throughput: Throughput,
    pub keyspace: Keyspace,
    pub command: Command,
    pub raw: Info,
}

impl Metric {
    pub fn latency(mut self, latency: u128) -> Self {
        self.status.latency = latency;
        self
    }

    pub fn calc_delta(mut self, other: Self, tick_rate: f64) -> Self {
        self.cpu.calc_delta(&other.cpu, tick_rate);
        self.throughput.calc_delta(&other.throughput, tick_rate);
        self.network.calc_delta(&other.network, tick_rate);
        self
    }
}

impl From<Info> for Metric {
    fn from(i: Info) -> Self {
        Metric {
            status: i.borrow().into(),
            network: i.borrow().into(),
            cpu: i.borrow().into(),
            memory: i.borrow().into(),
            throughput: i.borrow().into(),
            keyspace: i.borrow().into(),
            command: i.borrow().into(),
            raw: i,
        }
    }
}

trait Delta {
    fn calc_delta(&mut self, other: &Self, tick_rate: f64);
}