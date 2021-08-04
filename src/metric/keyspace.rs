use regex::Regex;

use crate::metric::Info;

lazy_static! {
    static ref REGEX: Regex = Regex::new("^db[0-9]+$").unwrap();
    static ref REGEX_VALUES: Regex = Regex::new("(?P<name>keys|expires|avg_ttl)=(?P<value>[0-9]+)").unwrap();
}

#[derive(Default)]
pub struct Keyspace {
    pub keyspace_hit_rate: f64,
    pub total_keys: u64,
    pub total_expires: u64,
    pub space: Vec<Space>,
}

pub struct Space {
    _name: String,
    keys: u64,
    expires: u64,
    _avg_ttl: u64,
}

impl From<&Info> for Keyspace {
    fn from(i: &Info) -> Self {
        let keyspace_hits = if let Some(keyspace_hits) = i.map.get("keyspace_hits") {
            keyspace_hits.parse::<f64>().unwrap_or(0.0)
        } else {
            0.0
        };

        let keyspace_misses = if let Some(keyspace_misses) = i.map.get("keyspace_misses") {
            keyspace_misses.parse::<f64>().unwrap_or(0.0)
        } else {
            0.0
        };

        let keyspace_hit_rate = keyspace_hits / (keyspace_hits + keyspace_misses);
        let keyspace_hit_rate = keyspace_hit_rate.max(0.0).min(100.0);

        let space = i.map.iter()
            .filter(|&it| { REGEX.captures(it.0).is_some() })
            .map(|it| (it.0, REGEX_VALUES.captures_iter(it.1)))
            .map(|mut it| {
                let keys = it.1.next().unwrap().name("value").map(|it| it.as_str()).unwrap_or("0");
                let expires = it.1.next().unwrap().name("value").map(|it| it.as_str()).unwrap_or("0");
                let avg_ttl = it.1.next().unwrap().name("value").map(|it| it.as_str()).unwrap_or("0");

                Space {
                    _name: it.0.to_owned(),
                    keys: keys.parse().unwrap_or(0),
                    expires: expires.parse().unwrap_or(0),
                    _avg_ttl: avg_ttl.parse().unwrap_or(0),
                }
            }).collect::<Vec<Space>>();

        Keyspace {
            keyspace_hit_rate,
            total_keys: space.iter().map(|it| it.keys).sum(),
            total_expires: space.iter().map(|it| it.expires).sum(),
            space,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;
    use std::collections::HashMap;

    use crate::metric::Info;
    use crate::metric::keyspace::{Keyspace, Space};

    #[test]
    fn simple() {
        let mut map = HashMap::new();
        map.insert("db0".to_owned(), "keys=3,expires=0,avg_ttl=0".to_owned());
        map.insert("db1".to_owned(), "keys=6,expires=0,avg_ttl=0".to_owned());
        map.insert("db10".to_owned(), "keys=7,expires=0,avg_ttl=0".to_owned());
        map.insert("db99999".to_owned(), "keys=7,expires=0,avg_ttl=0".to_owned());
        map.insert("db-1".to_owned(), "keys=7,expires=0,avg_ttl=0".to_owned());
        map.insert("other_key".to_owned(), "keys=7,expires=0,avg_ttl=0".to_owned());
        map.insert("used_cpu_sys".to_owned(), "33.888421".to_owned());

        let info = Info { map };
        let keyspace: Keyspace = info.borrow().into();
        assert_eq!(keyspace.space.len(), 4);
    }

    #[test]
    fn adv() {
        let mut map = HashMap::new();
        map.insert("db0".to_owned(), "keys=10,expires=2,avg_ttl=3".to_owned());
        map.insert("db1".to_owned(), "keys=40,expires=5,avg_ttl=6".to_owned());
        map.insert("db10".to_owned(), "keys=70,expires=8,avg_ttl=9".to_owned());
        map.insert("db99999".to_owned(), "keys=80,expires=11,avg_ttl=12".to_owned());
        map.insert("db_".to_owned(), "keys=13,expires=14,avg_ttl=15".to_owned());
        map.insert("used_cpu_sys".to_owned(), "33.888421".to_owned());

        let info = Info { map };
        let keyspace: Keyspace = info.borrow().into();

        assert_eq!(keyspace.space.len(), 4);
        assert_eq!(keyspace.space.iter().filter(|&it| it._name == "db99999".to_owned()).collect::<Vec<&Space>>().len(), 1);
        assert_eq!(keyspace.total_keys, 200);
        assert_eq!(keyspace.total_expires, 26);
    }
}