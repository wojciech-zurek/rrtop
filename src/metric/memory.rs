use crate::response::Info;

#[derive(Default)]
pub struct Memory {
    pub used_memory: u64,
    pub used_memory_rss: u64,
    pub max_memory: u64,
    pub mem_fragmentation_ratio: f32,
}

impl From<&Info> for Memory {
    fn from(i: &Info) -> Self {
        let used_memory = if let Some(used_memory) = i.map.get("used_memory") {
            used_memory.parse::<u64>().unwrap_or(0)
        } else {
            0
        };

        let used_memory_rss = if let Some(used_memory_rss) = i.map.get("used_memory_rss") {
            used_memory_rss.parse::<u64>().unwrap_or(0)
        } else {
            0
        };

        // max memory
        let max_memory = if let Some(max_memory) = i.map.get("maxmemory") {
            let max_memory = max_memory.parse::<u64>().unwrap_or(0);
            if max_memory > 0 {
                max_memory
            } else {
                if let Some(total_system_memory) = i.map.get("total_system_memory") {
                    total_system_memory.parse::<u64>().unwrap_or(0)
                } else {
                    0
                }
            }
        } else {
            0
        };

        let mem_fragmentation_ratio = if let Some(ratio) = i.map.get("mem_fragmentation_ratio") {
            ratio.parse::<f32>().unwrap_or(0.0)
        } else {
            0.0
        };

        Memory {
            used_memory,
            used_memory_rss,
            max_memory,
            mem_fragmentation_ratio,
        }
    }
}