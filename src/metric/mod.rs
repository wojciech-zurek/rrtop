use crate::metric::status::Status;
use crate::metric::network::Network;
use std::borrow::Borrow;
use crate::metric::cpu::Cpu;
use crate::metric::memory::Memory;
use crate::metric::throughput::Throughput;
use crate::response::Info;

pub mod status;
pub mod network;
pub mod cpu;
pub mod memory;
pub mod throughput;

#[derive(Default)]
pub struct Metric {
    pub status: Status,
    pub network: Network,
    pub cpu: Cpu,
    pub memory: Memory,
    pub throughput: Throughput,
}

impl Metric {
    pub fn latency(mut self, latency: u128) -> Self {
        self.status.latency = latency;
        self
    }

    pub fn calc_delta(mut self, other: Self, tick_rate: f64) -> Self {
        self.cpu.calc_delta(&other.cpu, tick_rate);
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
        }
    }
}

trait Delta {
    fn calc_delta(&mut self, other: &Self, tick_rate: f64);
}