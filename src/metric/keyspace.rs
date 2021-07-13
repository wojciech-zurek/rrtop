
use crate::response::Info;
use regex::Regex;

#[derive(Default)]
pub struct Keyspace {
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
        let regex = Regex::new("^db[0-9]+$").unwrap();
        let regex_values = Regex::new("(?P<name>keys|expires|avg_ttl)=(?P<value>[0-9]+)").unwrap();

        let space = i.0.iter()
            .filter(|&it| { regex.captures(it.0).is_some() })
            .map(|it| (it.0, regex_values.captures_iter(it.1)))
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
            total_keys: space.iter().map(|it| it.keys).sum(),
            total_expires: space.iter().map(|it| it.expires).sum(),
            space,
        }
    }
}

#[cfg(test)]
mod tests {
    use regex::{Regex};
    use std::collections::HashMap;
    use std::borrow::Borrow;
    use crate::metric::keyspace::{Space, Keyspace};
    use crate::response::Info;

    #[test]
    fn simple() {
        let _regex = Regex::new("^db[0-9]+$").unwrap();

        let mut map = HashMap::new();
        map.insert("db0".to_owned(), "keys=3,expires=0,avg_ttl=0".to_owned());
        map.insert("db1".to_owned(), "keys=6,expires=0,avg_ttl=0".to_owned());
        map.insert("db10".to_owned(), "keys=7,expires=0,avg_ttl=0".to_owned());
        map.insert("db99999".to_owned(), "keys=7,expires=0,avg_ttl=0".to_owned());
        map.insert("db-1".to_owned(), "keys=7,expires=0,avg_ttl=0".to_owned());
        map.insert("other_key".to_owned(), "keys=7,expires=0,avg_ttl=0".to_owned());
        map.insert("used_cpu_sys".to_owned(), "33.888421".to_owned());

        let info = Info(map);
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

        let info = Info(map);
        let keyspace: Keyspace = info.borrow().into();

        assert_eq!(keyspace.space.len(), 4);
        assert_eq!(keyspace.space.iter().filter(|&it| it._name == "db99999".to_owned()).collect::<Vec<&Space>>().len(), 1 );
        assert_eq!(keyspace.total_keys, 200);
        assert_eq!(keyspace.total_expires, 26);
    }
}