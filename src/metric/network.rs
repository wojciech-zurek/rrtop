use crate::response::Info;
use crate::metric::Delta;

#[derive(Default)]
pub struct Network {
    pub total_net_input_bytes: i64,
    pub total_net_output_bytes: i64,
    pub instantaneous_input_bps: u64,
    pub instantaneous_output_bps: u64,
    pub last_delta_network_input_bps: u64,
    pub last_delta_network_output_bps: u64,
}

impl Delta for Network {
    fn calc_delta(&mut self, other: &Self, tick_rate: f64) {
        self.last_delta_network_input_bps = if other.total_net_input_bytes == 0 {
            self.last_delta_network_input_bps
        } else {
            ((self.total_net_input_bytes as f64 - other.total_net_input_bytes as f64) / tick_rate).round() as u64
        };

        self.last_delta_network_output_bps = if other.total_net_output_bytes == 0 {
            self.last_delta_network_output_bps
        } else {
            ((self.total_net_output_bytes as f64 - other.total_net_output_bytes as f64) / tick_rate).round() as u64
        }
    }
}

impl From<&Info> for Network {
    fn from(i: &Info) -> Self {
        let total_net_input_bytes = if let Some(total_input) = i.0.get("total_net_input_bytes") {
            total_input.parse::<i64>().unwrap_or(0)
        } else {
            0
        };

        let total_net_output_bytes = if let Some(total_output) = i.0.get("total_net_output_bytes") {
            total_output.parse::<i64>().unwrap_or(0)
        } else {
            0
        };

        let instantaneous_input_bps = if let Some(input) = i.0.get("instantaneous_input_kbps") {
            input.parse::<f32>().unwrap_or(0.0) * 1000.0
        } else {
            0.0
        } as u64;


        let instantaneous_output_bps = if let Some(output) = i.0.get("instantaneous_output_kbps") {
            output.parse::<f32>().unwrap_or(0.0) * 1000.0
        } else {
            0.0
        } as u64;

        Network {
            total_net_input_bytes,
            total_net_output_bytes,
            instantaneous_input_bps,
            instantaneous_output_bps,
            last_delta_network_input_bps: instantaneous_input_bps,
            last_delta_network_output_bps: instantaneous_output_bps,
        }
    }
}