use crate::response::Info;

#[derive(Default)]
pub struct Server {
    pub uptime: i64,
    pub process_id: i64,
    pub latency: u128,
    pub version: String,
    pub role: String,
}

impl From<&Info> for Server {
    fn from(i: &Info) -> Self {
        let uptime = if let Some(u) = i.map.get("uptime_in_seconds") {
            u.parse::<i64>().unwrap_or(0)
        } else {
            0
        };

        let process_id = if let Some(pid) = i.map.get("process_id") {
            pid.parse::<i64>().unwrap_or(0)
        } else {
            0
        };

        let version = if let Some(rv) = i.map.get("redis_version") {
            rv
        } else {
            ""
        }.to_owned();

        let role = if let Some(r) = i.map.get("role") {
            r
        } else {
            ""
        }.to_owned();

        Server {
            uptime,
            process_id,
            latency: 0,
            version,
            role,
        }
    }
}