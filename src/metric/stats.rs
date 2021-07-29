use crate::metric::{Delta, Info};

#[derive(Default)]
pub struct Stats {
    pub total_commands_processed: u64,
    pub instantaneous_ops_per_sec: u64,
    pub last_delta_ops: f64,

    pub total_net_input_bytes: i64,
    pub total_net_output_bytes: i64,
    pub instantaneous_input_bps: u64,
    pub instantaneous_output_bps: u64,
    pub last_delta_network_input_bps: u64,
    pub last_delta_network_output_bps: u64,

    pub expired_keys: u64,
    pub last_delta_expired_keys: f64,
    pub evicted_keys: u64,
    pub last_delta_evicted_keys: f64,
}

impl Delta for Stats {
    fn calc_delta(&mut self, other: &Self, tick_rate: f64) {
        self.last_delta_ops = if other.total_commands_processed == 0 {
            self.last_delta_ops
        } else {
            (self.total_commands_processed as f64 - other.total_commands_processed as f64) / tick_rate
        };

        self.last_delta_network_input_bps = if other.total_net_input_bytes == 0 {
            self.last_delta_network_input_bps
        } else {
            ((self.total_net_input_bytes as f64 - other.total_net_input_bytes as f64) / tick_rate).round() as u64
        };

        self.last_delta_network_output_bps = if other.total_net_output_bytes == 0 {
            self.last_delta_network_output_bps
        } else {
            ((self.total_net_output_bytes as f64 - other.total_net_output_bytes as f64) / tick_rate).round() as u64
        };

        self.last_delta_expired_keys = if other.expired_keys == 0 {
            self.last_delta_expired_keys
        } else {
            (self.expired_keys as f64 - other.expired_keys as f64) / tick_rate
        };

        self.last_delta_evicted_keys = if other.evicted_keys == 0 {
            self.last_delta_evicted_keys
        } else {
            (self.evicted_keys as f64 - other.evicted_keys as f64) / tick_rate
        };
    }
}

impl From<&Info> for Stats {
    fn from(i: &Info) -> Self {
        let total_commands_processed = if let Some(total_commands) = i.map.get("total_commands_processed") {
            total_commands.parse::<u64>().unwrap_or(0)
        } else {
            0
        };

        let instantaneous_ops_per_sec = if let Some(ops_per_sec) = i.map.get("instantaneous_ops_per_sec") {
            ops_per_sec.parse::<u64>().unwrap_or(0)
        } else {
            0
        };

        let total_net_input_bytes = if let Some(total_input) = i.map.get("total_net_input_bytes") {
            total_input.parse::<i64>().unwrap_or(0)
        } else {
            0
        };

        let total_net_output_bytes = if let Some(total_output) = i.map.get("total_net_output_bytes") {
            total_output.parse::<i64>().unwrap_or(0)
        } else {
            0
        };

        let instantaneous_input_bps = if let Some(input) = i.map.get("instantaneous_input_kbps") {
            input.parse::<f32>().unwrap_or(0.0) * 1000.0
        } else {
            0.0
        } as u64;


        let instantaneous_output_bps = if let Some(output) = i.map.get("instantaneous_output_kbps") {
            output.parse::<f32>().unwrap_or(0.0) * 1000.0
        } else {
            0.0
        } as u64;

        let expired_keys = if let Some(expired_keys) = i.map.get("expired_keys") {
            expired_keys.parse::<u64>().unwrap_or(0)
        } else {
            0
        };

        let evicted_keys = if let Some(evicted_keys) = i.map.get("evicted_keys") {
            evicted_keys.parse::<u64>().unwrap_or(0)
        } else {
            0
        };

        Stats {
            total_commands_processed,
            instantaneous_ops_per_sec,
            last_delta_ops: instantaneous_ops_per_sec as f64,
            total_net_input_bytes,
            total_net_output_bytes,
            instantaneous_input_bps,
            instantaneous_output_bps,
            last_delta_network_input_bps: instantaneous_input_bps,
            last_delta_network_output_bps: instantaneous_output_bps,
            expired_keys,
            last_delta_expired_keys: 0.0,
            evicted_keys,
            last_delta_evicted_keys: 0.0,
        }
    }
}