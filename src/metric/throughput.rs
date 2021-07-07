use crate::response::Info;

#[derive(Default)]
pub struct Throughput {
    pub total_commands_processed: u64,
    pub instantaneous_ops_per_sec: u64,
}

impl From<&Info> for Throughput {
    fn from(i: &Info) -> Self {
        let total_commands_processed = if let Some(total_commands) = i.0.get("total_commands_processed") {
            total_commands.parse::<u64>().unwrap_or(0)
        } else {
            0
        };

        let instantaneous_ops_per_sec = if let Some(ops_per_sec) = i.0.get("instantaneous_ops_per_sec") {
            ops_per_sec.parse::<u64>().unwrap_or(0)
        } else {
            0
        };

        Throughput {
            total_commands_processed,
            instantaneous_ops_per_sec,
        }
    }
}