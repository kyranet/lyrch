extern crate postgres;

use serenity::prelude::TypeMapKey;
use std::env;
use std::sync::{Arc, Mutex};
use postgres::{Connection, TlsMode};

pub struct Settings(pub Arc<Mutex<Connection>>);

impl TypeMapKey for Settings {
    type Value = Settings;
}

impl Settings {
    pub fn new() -> Settings {
        let url = env::var("POSTGRES_URL").expect("Expected POSTGRES_URL to be set.");
        let connection = Connection::connect(url, TlsMode::None).unwrap();
        Settings(Arc::new(Mutex::new(connection)))
    }

    pub fn ensure_tables(&self) {
        let connection = self.0.lock().unwrap();
        connection.execute("CREATE TABLE IF NOT EXISTS users (
            id BIGINT PRIMARY KEY,
            banner_set CHARACTER VARYING(6),
            banner_list CHARACTER VARYING(6)[],
            badge_set CHARACTER VARYING(6)[],
            badge_list CHARACTER VARYING(6)[],
            color INTEGER DEFAULT 0,
            money_count INTEGER DEFAULT 0,
            point_count INTEGER DEFAULT 0,
            reputation_count INTEGER DEFAULT 0,
            next_daily TIMESTAMP,
            next_reputation TIMESTAMP
        )", &[]).unwrap();
        connection.execute("CREATE INDEX IF NOT EXISTS points ON ONLY users (
            point_count DESC
        )", &[]).unwrap();
    }
}
