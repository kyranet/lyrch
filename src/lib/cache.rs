use serenity::prelude::*;
use std::env;
use std::ops::DerefMut;
use std::sync::Arc;

pub struct RedisConnection {
    // client: Arc<Mutex<redis::Client>>,
    connection: Arc<Mutex<redis::Connection>>,
}

impl RedisConnection {
    pub fn new() -> Self {
        let url = &env::var("REDIS_URL").expect("Expected REDIS_URL to be set.")[..];
        let client = redis::Client::open(url).unwrap();
        let connection = client.get_connection().unwrap();
        Self {
            // client: Arc::new(Mutex::new(client)),
            connection: Arc::new(Mutex::new(connection)),
        }
    }

    pub fn query<T>(&self, cmd: &mut redis::Cmd) -> redis::RedisResult<T>
    where
        T: redis::FromRedisValue,
    {
        let mut conn = self.connection.lock();
        cmd.query(conn.deref_mut())
    }

    // pub fn execute(&mut self, cmd: &mut redis::Cmd) {
    //     let mut conn = self.connection.lock();
    //     cmd.execute(conn.deref_mut());
    // }

    pub fn set_ttl<K>(&self, key: K, value: K, ttl: u32)
    where
        K: redis::ToRedisArgs + Copy,
    {
        let mut conn = self.connection.lock();
        redis::pipe()
            .atomic()
            .add_command(redis::cmd("SET"))
            .arg(key)
            .arg(value)
            .ignore()
            .add_command(redis::cmd("EXPIRE"))
            .arg(key)
            .arg(ttl)
            .ignore()
            .execute(conn.deref_mut());
    }
}

impl TypeMapKey for RedisConnection {
    type Value = RedisConnection;
}
