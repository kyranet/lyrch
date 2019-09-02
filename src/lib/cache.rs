use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;
use serenity::prelude::*;
use std::env;
use std::ops::DerefMut;

pub struct RedisConnection(pub Pool<RedisConnectionManager>);

impl TypeMapKey for RedisConnection {
    type Value = RedisConnection;
}

impl RedisConnection {
    pub fn new() -> Self {
        let url = &env::var("REDIS_URL").expect("Expected REDIS_URL to be set.")[..];
        let manager = RedisConnectionManager::new(url).unwrap();
        let pool = Pool::new(manager).unwrap();
        Self(pool)
    }

    pub fn query<T>(&self, cmd: &mut redis::Cmd) -> redis::RedisResult<T>
    where
        T: redis::FromRedisValue,
    {
        let mut conn = self.0.clone().get().unwrap();
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
        let mut conn = self.0.clone().get().unwrap();
        redis::cmd("SET")
            .arg(key)
            .arg(value)
            .arg("EX")
            .arg(ttl)
            .execute(conn.deref_mut());
    }
}
