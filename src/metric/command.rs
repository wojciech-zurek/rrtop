use regex::Regex;

use crate::metric::Info;

lazy_static! {
    static ref REGEX: Regex = Regex::new("^cmdstat_[a-z]+$").unwrap();
    static ref REGEX_VALUES: Regex = Regex::new("(?P<name>calls|usec|usec_per_call)=(?P<value>[0-9.]+)").unwrap();
}

#[derive(Default)]
pub struct Command {
    pub stats: Vec<CmdStat>,
    pub sum_calls: f64,
    pub sum_usec: f64,
    pub sum_usec_per_call: f64,
}

#[derive(Clone)]
pub struct CmdStat {
    pub name: String,
    pub calls: u64,
    pub usec: u64,
    pub usec_per_call: f64,
}

impl From<&Info> for Command {
    fn from(i: &Info) -> Self {
        let stats = i.map.iter()
            .filter(|&it| { REGEX.captures(it.0).is_some() })
            .map(|it| (it.0, REGEX_VALUES.captures_iter(it.1)))
            .map(|mut it| {
                let calls = it.1.next().unwrap().name("value").map(|it| it.as_str()).unwrap_or("0");
                let usec = it.1.next().unwrap().name("value").map(|it| it.as_str()).unwrap_or("0");
                let usec_per_call = it.1.next().unwrap().name("value").map(|it| it.as_str()).unwrap_or("0.0");

                CmdStat {
                    name: it.0[8..].to_owned(),
                    calls: calls.parse().unwrap_or(0),
                    usec: usec.parse().unwrap_or(0),
                    usec_per_call: usec_per_call.parse().unwrap_or(0.0),
                }
            }).collect::<Vec<CmdStat>>();

        let sum_calls = stats.iter().map(|it| it.calls as f64).sum();
        let sum_usec = stats.iter().map(|it| it.usec as f64).sum();
        let sum_usec_per_call = stats.iter().map(|it| it.usec_per_call).sum();

        Command {
            stats,
            sum_calls,
            sum_usec,
            sum_usec_per_call,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;
    use std::collections::HashMap;

    use crate::metric::command::{CmdStat, Command};
    use crate::metric::Info;

    #[test]
    fn simple() {
        let mut map = HashMap::new();
        map.insert("cmdstat_ping".to_owned(), "calls=488938,usec=108907,usec_per_call=0.22".to_owned());
        map.insert("cmdstat".to_owned(), "keys=6,expires=0,avg_ttl=0".to_owned());
        map.insert("db1".to_owned(), "keys=6,expires=0,avg_ttl=0".to_owned());
        map.insert("db-1".to_owned(), "keys=7,expires=0,avg_ttl=0".to_owned());
        map.insert("other_key".to_owned(), "keys=7,expires=0,avg_ttl=0".to_owned());
        map.insert("used_cpu_sys".to_owned(), "33.888421".to_owned());

        let info = Info { map };
        let command: Command = info.borrow().into();
        assert_eq!(command.stats.len(), 1);
        assert_eq!(command.stats.first().unwrap().name, "ping".to_owned());
    }

    #[test]
    fn adv() {
        let mut map = HashMap::new();
        map.insert("cmdstat_ping".to_owned(), "calls=488938,usec=108907,usec_per_call=0.22".to_owned());
        map.insert("cmdstat_get".to_owned(), "calls=100008,usec=41754,usec_per_call=0.42".to_owned());
        map.insert("cmdstat_keys".to_owned(), "calls=3,usec=6170,usec_per_call=2056.67".to_owned());
        map.insert("db0".to_owned(), "keys=10,expires=2,avg_ttl=3".to_owned());
        map.insert("db_".to_owned(), "keys=13,expires=14,avg_ttl=15".to_owned());
        map.insert("used_cpu_sys".to_owned(), "33.888421".to_owned());

        let info = Info { map };
        let command: Command = info.borrow().into();

        assert_eq!(command.stats.len(), 3);
        assert_eq!(command.stats.iter().filter(|&it| it.name == "keys".to_owned()).collect::<Vec<&CmdStat>>().len(), 1);
        assert_eq!(command.stats.iter().map(|it| it.calls).sum::<u64>(), 588949);
        assert_eq!(command.stats.iter().map(|it| it.usec).sum::<u64>(), 156831);
        assert_eq!(command.stats.iter().map(|it| it.usec_per_call).sum::<f64>(), 2057.31);
    }
}