use std::collections::HashMap;
use redis::{FromRedisValue, RedisResult, Value, from_redis_value};

#[derive(Debug, PartialEq)]
pub struct Info(HashMap<String, String>);

impl FromRedisValue for Info {
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        let result: String = from_redis_value(v)?;

        let info = result
            .split("\r\n")
            .filter(|&it| it.len() > 0 && !it.starts_with("#"))
            .map(|it| it.split(":"))
            .map(|mut key| (key.next(), key.next()))
            .filter_map(|(key, value)| {
                if key.is_some() && value.is_some() {
                    Some((key.unwrap().to_owned(), value.unwrap().to_owned()))
                } else { None }
            })
            .collect::<HashMap<String, String>>();

        Ok(Info(info))
    }
}