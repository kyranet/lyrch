use std::env;
use std::sync::Arc;
use serenity::prelude::*;

pub struct RedisConnection(pub redis::Client, pub redis::Connection);

impl RedisConnection {
    pub fn new() -> Self {
        let url = &env::var("REDIS_URL").expect("Expected REDIS_URL to be set.")[..];
        let client = redis::Client::open(url).unwrap();
        let connection = client.get_connection().unwrap();
        Self(client, connection)
    }

    #[allow(dead_code)]
    pub fn query<T, F>(&mut self, command: &'static str, f: F) -> redis::RedisResult<T>
    where
        T: redis::FromRedisValue,
        F: FnOnce(redis::Cmd) -> redis::Cmd,
    {
        f(redis::cmd(command)).query(&mut self.1)
    }
}

impl TypeMapKey for RedisConnection {
    type Value = Arc<Mutex<RedisConnection>>;
}
