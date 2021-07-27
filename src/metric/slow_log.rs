pub struct SlowLog {
    pub logs: Vec<Log>,
}

pub struct Log {
    pub id: u64,
    pub timestamp: i64,
    pub exec_time: i64,
    pub command: String,
    pub client_ip: String,
    pub client_name: String,
}

impl From<Vec<Vec<(u64, i64, i64, Vec<String>, String, String)>>> for SlowLog {
    fn from(v: Vec<Vec<(u64, i64, i64, Vec<String>, String, String)>>) -> Self {
        let logs = v.into_iter()
            .flat_map(|it| it.into_iter())
            .map(|it| {
                let id = it.0;
                let timestamp = it.1;
                let exec_time = it.2;
                let command = it.3.join(" ");
                let client_ip = it.4;
                let client_name = it.5;

                Log {
                    id,
                    timestamp,
                    exec_time,
                    command,
                    client_ip,
                    client_name,
                }
            }).collect::<Vec<Log>>();

        SlowLog { logs }
    }
}