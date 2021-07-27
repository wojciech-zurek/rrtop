use std::collections::HashMap;

use redis::{from_redis_value, FromRedisValue, RedisResult, Value};

#[derive(Debug, Default, PartialEq)]
pub struct Info {
    pub map: HashMap<String, String>,
}

impl FromRedisValue for Info {
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        let result: String = from_redis_value(v)?;

        let map = result.lines()
            .filter(|&it| !it.is_empty() && !it.starts_with("#"))
            .map(|it| it.splitn(2, ":"))
            .map(|mut key| (key.next(), key.next()))
            .filter_map(|(key, value)| {
                if key.is_some() && value.is_some() {
                    Some((key.unwrap().to_owned(), value.unwrap().to_owned()))
                } else { None }
            })
            .collect::<HashMap<String, String>>();

        Ok(Info { map })
    }
}