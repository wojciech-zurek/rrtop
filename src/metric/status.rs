use crate::response::Info;

#[derive(Default)]
pub struct Status {
    pub uptime: i64,
    pub process_id: i64,
    pub latency: u128,
    pub version: String,
    pub role: String,
}

impl From<&Info> for Status {
    fn from(i: &Info) -> Self {
        let uptime = if let Some(u) = i.0.get("uptime_in_seconds") {
            u.parse::<i64>().unwrap_or(0)
        } else {
            0
        };

        let process_id = if let Some(pid) = i.0.get("process_id") {
            pid.parse::<i64>().unwrap_or(0)
        } else {
            0
        };

        let version = if let Some(rv) = i.0.get("redis_version") {
            rv
        } else {
            ""
        }.to_owned();

        let role = if let Some(r) = i.0.get("role") {
            r
        } else {
            ""
        }.to_owned();

        Status {
            uptime,
            process_id,
            latency: 0,
            version,
            role,
        }
    }
}